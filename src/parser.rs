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

    pub fn parse(&mut self) {
        let expr = self.expression();
        self.expressions.push(expr);

        if self.tokens[self.current].kind != TokenKind::Eof {
            self.parse()
        }
    }

    fn advance(&mut self) -> &Token {
        if self.tokens[self.current].kind != TokenKind::Eof {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, kind: TokenKind) -> Option<&Token> {
        // TODO handle error
        if kind == self.tokens[self.current].kind {
            Some(self.advance())
        } else {
            None
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

    fn expression(&mut self) -> Expr {
        self.term()
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.consume_match(&[TokenKind::Minus, TokenKind::Plus]) {
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

        while self.consume_match(&[TokenKind::Star, TokenKind::Slash]) {
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
        if self.consume_match(&[TokenKind::Minus]) {
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
            TokenKind::Number => {
                self.advance();
                Expr::Number {
                    value: self.tokens[self.current - 1].lexeme.to_owned(),
                }
            }
            TokenKind::LeftParen => {
                self.advance();
                let expression = Box::new(self.expression());
                self.consume(TokenKind::RightParen);
                Expr::Grouping { expression }
            }
            _ => todo!("Handle errors"),
        }
    }
}
