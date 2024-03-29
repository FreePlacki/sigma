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
        if self.tokens[self.current].kind == TokenKind::Import {
            self.import()
        } else {
            self.assignment()
        }
    }

    fn import(&mut self) -> Result<Expr, Error> {
        self.advance(); // consume import
        let file = self.advance();

        if &file.kind != &TokenKind::String {
            return Err(Error {
                line: self.tokens[self.current].line,
                pos: self.current - 1,
                kind: ErrorKind::ExpectedFilename,
            });
        }

        let file = file.lexeme.to_owned();
        Ok(Expr::Import { file })
    }

    fn assignment(&mut self) -> Result<Expr, Error> {
        let expr = self.term()?;

        if self.consume_match(&[TokenKind::Equals]) {
            let value = Box::new(self.term()?);

            return if let Expr::Variable { name } = expr {
                Ok(Expr::Assign { name, value })
            } else {
                Err(Error {
                    line: self.tokens[self.current].line,
                    pos: self.current - 1,
                    kind: ErrorKind::InvalidAssignment,
                })
            };
        }

        Ok(expr)
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
        let mut expr = self.exponent()?;

        while self.consume_match(&[TokenKind::Star, TokenKind::Slash]) {
            let operator = self.tokens[self.current - 1].to_owned();
            let right = self.exponent()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn exponent(&mut self) -> Result<Expr, Error> {
        let expr = self.unary()?;

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

    fn unary(&mut self) -> Result<Expr, Error> {
        if self.consume_match(&[TokenKind::Minus]) {
            let operator = self.tokens[self.current - 1].to_owned();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }
        self.factorial()
    }

    fn factorial(&mut self) -> Result<Expr, Error> {
        let expr = self.call()?;

        if self.consume_match(&[TokenKind::Bang]) {
            let operator = self.tokens[self.current - 1].to_owned();
            return Ok(Expr::Unary {
                operator,
                right: Box::new(expr),
            });
        }
        Ok(expr)
    }

    fn call(&mut self) -> Result<Expr, Error> {
        // NOTE: We can safely do current + 1 because there is always an EOF token
        if self.tokens[self.current + 1].kind != TokenKind::LeftParen {
            return self.primary();
        }

        let name = self.consume(TokenKind::Identifier, ErrorKind::ExpectedFunctionName)?;
        let name = name.to_owned();
        self.advance(); // consume '('

        let mut arguments = Vec::<Expr>::new();
        if self.tokens[self.current].kind != TokenKind::RightParen {
            loop {
                let argument = self.term()?;
                arguments.push(argument);

                if self.tokens[self.current].kind == TokenKind::RightParen {
                    break;
                }

                self.consume(TokenKind::Comma, ErrorKind::MissingComma)?;
            }
        }
        self.consume(TokenKind::RightParen, ErrorKind::MissingRightParen)?;

        Ok(Expr::Call { name, arguments })
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        self.advance();

        match self.tokens[self.current - 1].kind {
            TokenKind::Number => {
                let number_pos = self.current - 1;
                let dimension = if self.tokens[self.current].kind == TokenKind::LeftBracket {
                    self.advance();
                    let expr = Box::new(self.expression()?);
                    self.consume(TokenKind::RightBracket, ErrorKind::MissingRightBracket)?;
                    Some(expr)
                } else {
                    None
                };

                Ok(Expr::Number {
                    value: self.tokens[number_pos].lexeme.to_owned(),
                    dimension,
                })
            }
            TokenKind::LeftParen => {
                let expression = Box::new(self.expression()?);
                self.consume(TokenKind::RightParen, ErrorKind::MissingRightParen)?;
                Ok(Expr::Grouping { expression })
            }
            TokenKind::Identifier => Ok(Expr::Variable {
                name: self.tokens[self.current - 1].clone(),
            }),
            _ => Err(Error {
                line: self.tokens[self.current - 1].line,
                pos: self.current,
                kind: ErrorKind::ExpectedExpression,
            }),
        }
    }
}
