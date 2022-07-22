use crate::tokens::{Token, TokenType};

pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 0,
            tokens: Vec::<Token>::new(),
        }
    }

    pub fn scan(&mut self) -> &Vec<Token> {
        while self.current < self.source.len() {
            self.start = self.current;
            self.scan_token();
        }

        &self.tokens
    }

    fn scan_token(&mut self) {
        // use macro to have optional argument
        macro_rules! add_token {
            ($type: expr, $lexeme: expr) => {
                self.tokens.push(Token {
                    kind: $type,
                    lexeme: $lexeme,
                    line: self.line,
                    start: self.current - 1,
                })
            };
            ($type: expr) => {
                self.tokens.push(Token {
                    kind: $type,
                    lexeme: self.source.chars().nth(self.current).unwrap().to_string(),
                    line: self.line,
                    start: self.current,
                })
            };
        }

        let c = self.advance();

        match c {
            '(' => add_token!(TokenType::LeftParen),
            ')' => add_token!(TokenType::RightParen),
            '-' => add_token!(TokenType::Minus),
            '+' => add_token!(TokenType::Plus),
            '*' => add_token!(TokenType::Star),
            '/' => add_token!(TokenType::Slash),
            '^' => add_token!(TokenType::Caret),
            '!' => add_token!(TokenType::Bang),
            '=' => add_token!(TokenType::Equals),

            '0'..='9' | '.' => {
                while self.peek().is_ascii_digit() {
                    self.advance();
                }
                // TODO: allow more than one '_' and ','
                if ['.', ',', '_'].contains(&self.peek()) && self.peek_next().is_ascii_digit() {
                    self.advance();

                    while self.peek().is_ascii_digit() {
                        self.advance();
                    }
                }
                add_token!(
                    TokenType::Number,
                    self.source[self.start..self.current].to_string()
                )
            }

            '#' => {
                while !['\n', '\0'].contains(&self.peek()) {
                    self.advance();
                }
                // consume '\n'
                self.advance();
                self.line += 1;
            }
            _ => { //TODO: handle
            }
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn peek(&self) -> char {
        if let Some(c) = self.source.chars().nth(self.current) {
            c
        } else {
            '\0'
        }
    }

    fn peek_next(&self) -> char {
        if let Some(c) = self.source.chars().nth(self.current + 1) {
            c
        } else {
            '\0'
        }
    }
}
