use crate::error::{Error, ErrorKind};
use crate::expr::Expr;
use crate::tokens::{Token, TokenKind};

pub struct Interpreter {
    expressions: Vec<Expr>,
}

impl Interpreter {
    pub fn new(expressions: Vec<Expr>) -> Self {
        Self { expressions }
    }

    pub fn interpret(&self) -> Result<(), Error> {
        for expr in &self.expressions {
            let res = self.evaluate(expr)?;
            println!("{}", res);
        }
        Ok(())
    }

    fn evaluate(&self, expr: &Expr) -> Result<f64, Error> {
        match expr {
            Expr::Number { value } => self.eval_number(value),
            Expr::Unary { operator, right } => self.eval_unary(operator, *right.to_owned()),
            Expr::Binary {
                left,
                operator,
                right,
            } => self.eval_binary(*left.to_owned(), operator, *right.to_owned()),
            Expr::Grouping { expression } => self.evaluate(expression),
        }
    }

    fn eval_number(&self, value: &str) -> Result<f64, Error> {
        // TODO support XeY
        let num = value.replace(&['_', ','], "");
        Ok(num.parse().unwrap())
    }

    fn eval_unary(&self, oper: &Token, right: Expr) -> Result<f64, Error> {
        let num = self.evaluate(&right)?;
        match oper.kind {
            TokenKind::Minus => Ok(-1.0 * num),
            TokenKind::Bang => match self.factorial(num) {
                Ok(res) => Ok(res),
                Err(kind) => Err(Error {
                    line: oper.line,
                    pos: oper.start,
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

    fn eval_binary(&self, left: Expr, oper: &Token, right: Expr) -> Result<f64, Error> {
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
                        pos: oper.start,
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
}
