use crate::error::{error, ErrorType};
use crate::expr::Expr;
use crate::tokens::{Token, TokenType};

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

    pub fn parse(&mut self) {
        let expr = self.expression();
        self.expressions.push(expr);

        if self.tokens[self.current].kind != TokenType::Eof {
            self.parse()
        }
    }

    fn advance(&mut self) -> &Token {
        if self.tokens[self.current].kind != TokenType::Eof {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, kind: TokenType) -> Option<&Token> {
        // TODO handle error
        if kind == self.tokens[self.current].kind {
            Some(self.advance())
        } else {
            None
        }
    }

    fn consume_match(&mut self, token_kinds: &[TokenType]) -> bool {
        for kind in token_kinds {
            if &self.tokens[self.current].kind == kind {
                self.advance();
                return true;
            }
        }

        false
    }

    fn expression(&mut self) -> Expr {
        self.term()
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.consume_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.tokens[self.current - 1].to_owned();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.consume_match(&[TokenType::Star, TokenType::Slash]) {
            let operator = self.tokens[self.current - 1].to_owned();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.consume_match(&[TokenType::Minus]) {
            let operator = self.tokens[self.current - 1].to_owned();
            let right = self.unary();
            return Expr::Unary {
                operator,
                right: Box::new(right),
            };
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        match self.tokens[self.current].kind {
            TokenType::Number => {
                self.advance();
                Expr::Number {
                    value: self.tokens[self.current - 1].lexeme.to_owned(),
                }
            }
            TokenType::LeftParen => {
                self.advance();
                let expression = Box::new(self.expression());
                self.consume(TokenType::RightParen);
                Expr::Grouping { expression }
            }
            _ => todo!("Handle errors"),
        }
    }
}
