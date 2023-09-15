use std::collections::HashMap;

use crate::error::{Error, ErrorKind};
use crate::expr::Expr;
use crate::tokens::{Token, TokenKind};
use crate::value::{Dimension, Value};

pub type Environment = HashMap<String, Value>;

pub struct Interpreter {
    expressions: Vec<Expr>,
    environment: Environment,
}

macro_rules! gen_error {
    ($kind:expr, $oper:expr) => {
        Error {
            line: $oper.line,
            pos: $oper.pos,
            kind: $kind,
        }
    };
}

impl Interpreter {
    pub fn new(expressions: Vec<Expr>, environment: Environment) -> Self {
        Self {
            expressions,
            environment,
        }
    }

    pub fn interpret(&mut self, is_repl: bool) -> Result<Environment, Error> {
        for expr in &self.expressions.clone() {
            let res = self.evaluate(expr)?;
            match &expr {
                Expr::Assign { .. } if !is_repl => continue,
                Expr::Variable { name } if !is_repl => print!("{} = ", name.lexeme),
                _ => {}
            }
            let formatted_num = if res.number.abs() > 1e4 || res.number.abs() < 1e-4 {
                format!("{:e}", res.number)
            } else {
                res.number.to_string()
            };
            if let Some(dim) = res.dimension {
                println!("{formatted_num} [{}]", dim.lexeme);
            } else {
                println!("{formatted_num}");
            }
        }
        Ok(self.environment.clone())
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, Error> {
        match expr {
            Expr::Number { value, dimension } => self.eval_number(value, dimension.to_owned()),
            Expr::Unary { operator, right } => self.eval_unary(operator, *right.to_owned()),
            Expr::Binary {
                left,
                operator,
                right,
            } => self.eval_binary(*left.to_owned(), operator, *right.to_owned()),
            Expr::Grouping { expression } => self.evaluate(expression),
            Expr::Variable { name } => self.eval_variable(name.to_owned()),
            Expr::Assign { name, value } => self.eval_assign(name.to_owned(), *value.to_owned()),
        }
    }

    fn eval_number(&mut self, value: &str, dimension: Option<Box<Expr>>) -> Result<Value, Error> {
        let number = value.replace(['_', ','], "");
        let mut s = number.split('e');
        let mut number = s.next().unwrap().parse().unwrap();
        if let Some(exp) = s.next() {
            let exp = exp.parse().unwrap();
            number *= 10f64.powi(exp);
        }

        let dimension = if let Some(dim) = dimension {
            Some(self.eval_dimension(&dim)?)
        } else {
            None
        };
        Ok(Value { number, dimension })
    }

    fn eval_dimension(&mut self, dimension: &Expr) -> Result<Dimension, Error> {
        match dimension {
            Expr::Binary {
                left,
                operator,
                right,
            } => self.eval_binary_dim(*left.to_owned(), operator, *right.to_owned()),
            Expr::Grouping { expression } => self.eval_dimension(expression),
            Expr::Variable { name } => self.eval_variable_dim(name.to_owned()),
            _ => todo!("Throw an error or sf"),
        }
    }

    fn eval_binary_dim(
        &mut self,
        left: Expr,
        oper: &Token,
        right: Expr,
    ) -> Result<Dimension, Error> {
        let left = self.eval_dimension(&left)?;
        if oper.kind == TokenKind::Caret {
            let right = self.evaluate(&right)?;
            return Ok(left.pow_dim(right.number));
        }
        let right = self.eval_dimension(&right)?;

        match oper.kind {
            TokenKind::Plus => {
                if !left.check(Some(&right)) {
                    return Err(gen_error!(ErrorKind::InvalidUnitsAdd, oper));
                }
                Ok(left)
            }
            TokenKind::Minus => {
                if !left.check(Some(&right)) {
                    return Err(gen_error!(ErrorKind::InvalidUnitsSub, oper));
                }
                Ok(left)
            }
            TokenKind::Star => Ok(left.mul_dim(&right)),
            TokenKind::Slash => Ok(left.div_dim(&right)),
            _ => unreachable!(),
        }
    }

    fn eval_variable_dim(&self, name: Token) -> Result<Dimension, Error> {
        Ok(Dimension::new(name.lexeme))
    }

    fn eval_unary(&mut self, oper: &Token, right: Expr) -> Result<Value, Error> {
        let right = self.evaluate(&right)?;
        match oper.kind {
            TokenKind::Minus => Ok(Value {
                number: -1.0 * right.number,
                dimension: right.dimension,
            }),
            TokenKind::Bang => match self.factorial(right) {
                Ok(res) => Ok(res),
                Err(kind) => Err(gen_error!(kind, oper)),
            },
            _ => unreachable!(),
        }
    }

    fn factorial(&self, value: Value) -> Result<Value, ErrorKind> {
        if value.number < 0.0 {
            return Err(ErrorKind::FactorialDomain);
        }
        if let Some(dim) = value.dimension {
            if !dim.is_dimensionless() {
                return Err(ErrorKind::FactorialDimension);
            }
        }
        let mut res = 1.0;
        for i in 2..=value.number.round() as usize {
            res *= i as f64;
        }
        Ok(Value {
            number: res,
            dimension: None,
        })
    }

    fn eval_binary(&mut self, left: Expr, oper: &Token, right: Expr) -> Result<Value, Error> {
        let left = self.evaluate(&left)?;
        let right = self.evaluate(&right)?;

        let check_left = left.dimension.is_some()
            && left
                .dimension
                .as_ref()
                .unwrap()
                .check(right.dimension.as_ref());
        let check_right = right.dimension.is_some()
            && right
                .dimension
                .as_ref()
                .unwrap()
                .check(left.dimension.as_ref());
        let both_none = left.dimension.is_none() && right.dimension.is_none();

        match oper.kind {
            TokenKind::Plus => {
                let number = left.number + right.number;

                if check_left || check_right || both_none {
                    Ok(Value {
                        number,
                        dimension: if check_left {
                            left.dimension
                        } else {
                            right.dimension
                        },
                    })
                } else {
                    Err(gen_error!(ErrorKind::InvalidUnitsAdd, oper))
                }
            }
            TokenKind::Minus => {
                let number = left.number - right.number;

                if check_left || check_right || both_none {
                    Ok(Value {
                        number,
                        dimension: if check_left {
                            left.dimension
                        } else {
                            right.dimension
                        },
                    })
                } else {
                    Err(gen_error!(ErrorKind::InvalidUnitsSub, oper))
                }
            }
            TokenKind::Star => {
                let number = left.number * right.number;
                let dimension = match (left.dimension, right.dimension) {
                    (Some(left_dim), Some(right_dim)) => Some(left_dim.mul_dim(&right_dim)),
                    (Some(left_dim), _) => Some(left_dim),
                    (_, Some(right_dim)) => Some(right_dim),
                    _ => None,
                };

                Ok(Value { number, dimension })
            }
            TokenKind::Slash => {
                if right.number == 0.0 {
                    return Err(gen_error!(ErrorKind::DivisionByZero, oper));
                }
                let number = left.number / right.number;
                let dimension = match (left.dimension, right.dimension) {
                    (Some(left_dim), Some(right_dim)) => Some(left_dim.div_dim(&right_dim)),
                    (Some(left_dim), _) => Some(left_dim),
                    (_, Some(right_dim)) => Some(right_dim.pow_dim(-1.0)),
                    _ => None,
                };

                Ok(Value { number, dimension })
            }
            TokenKind::Caret => {
                let number = left.number.powf(right.number);
                match (&left.dimension, &right.dimension) {
                    (_, Some(right_dim)) => {
                        if !right_dim.is_dimensionless() {
                            Err(gen_error!(ErrorKind::InvalidUnitsPow, oper))
                        } else {
                            Ok(Value {
                                number,
                                dimension: left.dimension,
                            })
                        }
                    }
                    (Some(left_dim), None) => {
                        let dimension = Some(left_dim.pow_dim(right.number));
                        Ok(Value { number, dimension })
                    }
                    _ => Ok(Value {
                        number,
                        dimension: None,
                    }),
                }
            }
            _ => unreachable!(),
        }
    }

    fn eval_variable(&mut self, name: Token) -> Result<Value, Error> {
        if let Some(expr) = self.environment.get(&name.lexeme) {
            Ok(expr.clone())
        } else {
            Err(gen_error!(ErrorKind::UndefinedVariable, name))
        }
    }

    fn eval_assign(&mut self, name: Token, value: Expr) -> Result<Value, Error> {
        let value = self.evaluate(&value)?;
        self.environment.insert(name.lexeme, value.clone());
        Ok(value)
    }
}
