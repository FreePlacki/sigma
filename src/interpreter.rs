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

impl Interpreter {
    pub fn new(expressions: Vec<Expr>, environment: Environment) -> Self {
        Self {
            expressions,
            environment,
        }
    }

    pub fn interpret(&mut self) -> Result<Environment, Error> {
        for expr in &self.expressions.clone() {
            let res = self.evaluate(expr)?;
            println!("{}", res.number);
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

    fn eval_number(&self, value: &str, dimension: Option<Box<Expr>>) -> Result<Value, Error> {
        // TODO support XeY
        let number = value.replace(&['_', ','], "").parse().unwrap();
        let dimension = if let Some(dim) = dimension {
            Some(self.eval_dimension(&dim)?)
        } else {
            None
        };
        Ok(Value { number, dimension })
    }

    fn eval_dimension(&self, dimension: &Expr) -> Result<Dimension, Error> {
        match dimension {
            // TODO Numbers for exponents
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

    fn eval_binary_dim(&self, left: Expr, oper: &Token, right: Expr) -> Result<Dimension, Error> {
        let mut left = self.eval_dimension(&left)?;
        let right = self.eval_dimension(&right)?;

        match oper.kind {
            TokenKind::Plus => {
                if !left.check(&right) {
                    return Err(Error {
                        line: oper.line,
                        pos: oper.pos,
                        kind: ErrorKind::InvalidUnitsAdd,
                    });
                }
                Ok(left)
            }
            TokenKind::Minus => {
                if !left.check(&right) {
                    return Err(Error {
                        line: oper.line,
                        pos: oper.pos,
                        kind: ErrorKind::InvalidUnitsSub,
                    });
                }
                Ok(left)
            }
            TokenKind::Star => Ok(left.mul_dim(&right)),
            TokenKind::Slash => Ok(left.div_dim(&right)),
            TokenKind::Caret => {
                // TODO has to be done differently to work with numbers
                // idea: move it before right to parse right as a number
                todo!()
            }
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
                Err(kind) => Err(Error {
                    line: oper.line,
                    pos: oper.pos,
                    kind,
                }),
            },
            _ => unreachable!(),
        }
    }

    fn factorial(&self, value: Value) -> Result<Value, ErrorKind> {
        if value.number < 0.0 {
            return Err(ErrorKind::FactorialDomain);
        }
        if value.dimension.is_some() {
            return Err(ErrorKind::FactorialDimension);
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

        match oper.kind {
            TokenKind::Plus => {
                // rn only checking if both have some dim, consider throwing error when only one
                // has a dimension (ex: is 1 [kg] + 2 valid?)
                if let (Some(left_dim), Some(right_dim)) = (&left.dimension, &right.dimension) {
                    if !left_dim.check(right_dim) {
                        // TODO make a macro for errors
                        return Err(Error {
                            line: oper.line,
                            pos: oper.pos,
                            kind: ErrorKind::InvalidUnitsAdd,
                        });
                    }
                }
                Ok(Value {
                    number: left.number + right.number,
                    dimension: left.dimension,
                })
            }
            TokenKind::Minus => {
                if let (Some(left_dim), Some(right_dim)) = (&left.dimension, &right.dimension) {
                    if !left_dim.check(right_dim) {
                        return Err(Error {
                            line: oper.line,
                            pos: oper.pos,
                            kind: ErrorKind::InvalidUnitsSub,
                        });
                    }
                }
                Ok(Value {
                    number: left.number - right.number,
                    dimension: left.dimension,
                })
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
                    Err(Error {
                        line: oper.line,
                        pos: oper.pos,
                        kind: ErrorKind::DivisionByZero,
                    })
                } else {
                    let number = left.number / right.number;
                    let dimension = match (left.dimension, right.dimension) {
                        (Some(left_dim), Some(right_dim)) => Some(left_dim.div_dim(&right_dim)),
                        (Some(left_dim), _) => Some(left_dim),
                        (_, Some(right_dim)) => Some(right_dim),
                        _ => None,
                    };

                    Ok(Value { number, dimension })
                }
            }
            TokenKind::Caret => {
                let dimension = left.dimension.map(|dim| dim.pow_dim(right.number));

                Ok(Value {
                    number: left.number.powf(right.number),
                    dimension,
                })
            }
            _ => unreachable!(),
        }
    }

    fn eval_variable(&mut self, name: Token) -> Result<Value, Error> {
        if let Some(expr) = self.environment.get(&name.lexeme) {
            Ok(expr.clone())
        } else {
            Err(Error {
                line: name.line,
                pos: name.pos,
                kind: ErrorKind::UndefinedVariable,
            })
        }
    }

    fn eval_assign(&mut self, name: Token, value: Expr) -> Result<Value, Error> {
        let value = self.evaluate(&value)?;
        self.environment.insert(name.lexeme, value.clone());
        Ok(value)
    }
}
