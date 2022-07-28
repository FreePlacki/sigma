use crate::error::{Error, ErrorKind};
use crate::expr::Expr;
use crate::tokens::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    pub expressions: Vec<Expr>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            expressions: vec![],
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<&Vec<Expr>, Error> {
        let expr = self.expression()?;
        self.expressions.push(expr);

        if self.tokens[self.current].kind != TokenKind::Eof {
            self.parse()
        } else {
            Ok(&self.expressions)
        }
    }

    fn advance(&mut self) -> &Token {
        if self.tokens[self.current].kind != TokenKind::Eof {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, kind: TokenKind, error_kind: ErrorKind) -> Result<&Token, Error> {
        let token = &self.tokens[self.current];

        if kind == token.kind {
            Ok(self.advance())
        } else {
            Err(Error {
                line: token.line,
                pos: self.current + 1,
                kind: error_kind,
            })
        }
    }

    fn consume_match(&mut self, token_kinds: &[TokenKind]) -> bool {
        if token_kinds
            .iter()
            .any(|kind| kind == &self.tokens[self.current].kind)
        {
            self.advance();
            return true;
        }

        false
    }

    fn expression(&mut self) -> Result<Expr, Error> {
        self.term()
    }

    fn term(&mut self) -> Result<Expr, Error> {
        let mut expr = self.factor()?;

        while self.consume_match(&[TokenKind::Minus, TokenKind::Plus]) {
            let operator = self.tokens[self.current - 1].to_owned();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, Error> {
        let mut expr = self.unary()?;

        while self.consume_match(&[TokenKind::Star, TokenKind::Slash]) {
            let operator = self.tokens[self.current - 1].to_owned();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        if self.consume_match(&[TokenKind::Minus]) {
            let operator = self.tokens[self.current - 1].to_owned();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }
        self.exponent()
    }

    fn exponent(&mut self) -> Result<Expr, Error> {
        let expr = self.factorial()?;

        if self.consume_match(&[TokenKind::Caret]) {
            let operator = self.tokens[self.current - 1].to_owned();
            let right = self.exponent()?;
            return Ok(Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn factorial(&mut self) -> Result<Expr, Error> {
        let expr = self.primary()?;

        if self.consume_match(&[TokenKind::Bang]) {
            let operator = self.tokens[self.current - 1].to_owned();
            return Ok(Expr::Unary {
                operator,
                right: Box::new(expr),
            });
        }
        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        let token = &self.tokens[self.current];
        match token.kind {
            TokenKind::Number => {
                self.advance();
                Ok(Expr::Number {
                    value: self.tokens[self.current - 1].lexeme.to_owned(),
                })
            }
            TokenKind::LeftParen => {
                self.advance();
                let expression = Box::new(self.expression()?);
                self.consume(TokenKind::RightParen, ErrorKind::MissingRightParen)?;
                Ok(Expr::Grouping { expression })
            }
            _ => Err(Error {
                line: token.line,
                pos: self.current,
                kind: ErrorKind::ExpectedExpression,
            }),
        }
    }
}
