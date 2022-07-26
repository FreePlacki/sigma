use crate::error::Error;
use crate::expr::Expr;
use crate::tokens::Token;

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
            _ => {
                Ok(0.0) // TODO handle errors
            }
        }
    }

    fn eval_number(&self, value: &str) -> Result<f64, Error> {
        // TODO handle unwrap
        // TODO support XeY
        let num = value.replace(&['_', ','], "");
        Ok(num.parse().unwrap())
    }

    fn eval_unary(&self, _oper: &Token, right: Expr) -> Result<f64, Error> {
        let num = self.evaluate(&right)?;
        Ok(-1.0 * num)
    }
}
