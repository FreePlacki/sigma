use std::collections::HashMap;

use crate::error::{Error, ErrorKind};
use crate::expr::Expr;
use crate::tokens::{Token, TokenKind};

pub type Environment = HashMap<String, Expr>;

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
            println!("{}", res);
        }
        Ok(self.environment.clone())
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<f64, Error> {
        match expr {
            Expr::Number { value } => self.eval_number(value),
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

    fn eval_number(&self, value: &str) -> Result<f64, Error> {
        // TODO support XeY
        let num = value.replace(&['_', ','], "");
        Ok(num.parse().unwrap())
    }

    fn eval_unary(&mut self, oper: &Token, right: Expr) -> Result<f64, Error> {
        let num = self.evaluate(&right)?;
        match oper.kind {
            TokenKind::Minus => Ok(-1.0 * num),
            TokenKind::Bang => match self.factorial(num) {
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

    fn factorial(&self, number: f64) -> Result<f64, ErrorKind> {
        if number < 0.0 {
            return Err(ErrorKind::FactorialDomain);
        }
        let mut res = 1.0;
        for i in 2..=number.round() as usize {
            res *= i as f64;
        }
        Ok(res)
    }

    fn eval_binary(&mut self, left: Expr, oper: &Token, right: Expr) -> Result<f64, Error> {
        let left = self.evaluate(&left)?;
        let right = self.evaluate(&right)?;

        match oper.kind {
            TokenKind::Plus => Ok(left + right),
            TokenKind::Minus => Ok(left - right),
            TokenKind::Star => Ok(left * right),
            TokenKind::Slash => {
                if right == 0.0 {
                    Err(Error {
                        line: oper.line,
                        pos: oper.pos,
                        kind: ErrorKind::DivisionByZero,
                    })
                } else {
                    Ok(left / right)
                }
            }
            TokenKind::Caret => Ok(left.powf(right)),
            _ => unreachable!(),
        }
    }

    fn eval_variable(&mut self, name: Token) -> Result<f64, Error> {
        if let Some(expr) = self.environment.get(&name.lexeme) {
            self.evaluate(&expr.clone())
        } else {
            Err(Error {
                line: name.line,
                pos: name.pos,
                kind: ErrorKind::UndefinedVariable,
            })
        }
    }

    fn eval_assign(&mut self, name: Token, value: Expr) -> Result<f64, Error> {
        self.environment.insert(name.lexeme, value.clone());
        self.evaluate(&value)
    }
}
