use crate::error::{Error, ErrorKind};
use crate::tokens::{Token, TokenKind};

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

    pub fn scan(&mut self) -> Result<&Vec<Token>, Error> {
        while self.current < self.source.len() {
            self.start = self.current;
            self.scan_token()?
        }

        self.tokens.push(Token {
            kind: TokenKind::Eof,
            lexeme: "".to_string(),
            line: self.line,
            start: self.current,
        });
        Ok(&self.tokens)
    }

    fn scan_token(&mut self) -> Result<(), Error> {
        // use macro to have optional argument
        macro_rules! add_token {
            ($kind: expr, $lexeme: expr) => {
                self.tokens.push(Token {
                    kind: $kind,
                    lexeme: $lexeme,
                    line: self.line,
                    start: self.start,
                })
            };
            ($kind: expr) => {
                self.tokens.push(Token {
                    kind: $kind,
                    lexeme: self
                        .source
                        .chars()
                        .nth(self.current - 1)
                        .unwrap()
                        .to_string(),
                    line: self.line,
                    start: self.start,
                })
            };
        }

        let c = self.advance();

        match c {
            '(' => add_token!(TokenKind::LeftParen),
            ')' => add_token!(TokenKind::RightParen),
            '-' => add_token!(TokenKind::Minus),
            '+' => add_token!(TokenKind::Plus),
            '*' => add_token!(TokenKind::Star),
            '/' => add_token!(TokenKind::Slash),
            '^' => add_token!(TokenKind::Caret),
            '!' => add_token!(TokenKind::Bang),
            '=' => add_token!(TokenKind::Equals),

            '0'..='9' | '.' => {
                while self.peek().is_ascii_digit()
                    || ([',', '_'].contains(&self.peek()) && self.peek_next().is_ascii_digit())
                {
                    self.advance();
                }

                if self.peek() == '.' && self.peek_next().is_ascii_digit() {
                    self.advance();

                    while self.peek().is_ascii_digit() {
                        self.advance();
                    }
                }

                let lexeme = self.source[self.start..self.current].to_string();
                if lexeme != "." {
                    add_token!(TokenKind::Number, lexeme)
                } else {
                    return Err(Error {
                        line: self.line,
                        pos: self.current,
                        kind: ErrorKind::UnexpectedCharacter,
                    });
                }
            }

            '#' => {
                while !['\n', '\0'].contains(&self.peek()) {
                    self.advance();
                }
                // consume '\n'
                self.advance();
                self.line += 1;
            }

            '\t' => add_token!(TokenKind::Tab),
            ' ' | '\r' => {}
            '\n' => self.line += 1,

            'a'..='z' | 'A'..='Z' => {
                while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
                    self.advance();
                }

                add_token!(
                    TokenKind::Identifier,
                    self.source[self.start..self.current].to_string()
                )
            }

            _ => {
                return Err(Error {
                    line: self.line,
                    pos: self.current - 1, // -1 cause we advanced earlier
                    kind: ErrorKind::UnexpectedCharacter,
                });
            }
        }

        Ok(())
    }

    fn advance(&mut self) -> char {
        let c = self
            .source
            .chars()
            .nth(self.current)
            .expect("Cannot advance if current not in [0, len(source)]");
        self.current += 1;
        c
    }

    fn peek(&self) -> char {
        assert!(self.current <= self.source.len());
        if let Some(c) = self.source.chars().nth(self.current) {
            c
        } else {
            '\0'
        }
    }

    fn peek_next(&self) -> char {
        assert!(self.current < self.source.len());
        if let Some(c) = self.source.chars().nth(self.current + 1) {
            c
        } else {
            '\0'
        }
    }
}

#[cfg(test)]
mod scanner_tests {
    use super::*;

    #[test]
    fn advance_test() {
        let source = "12";
        let mut scanner = Scanner::new(source.into());
        let a = scanner.advance();
        let b = scanner.advance();
        assert_eq!(a, '1');
        assert_eq!(b, '2');
        assert_eq!(scanner.current, 2);
    }

    #[test]
    #[should_panic]
    fn invalid_advance() {
        let source = "";
        let mut scanner = Scanner::new(source.into());
        let _ = scanner.advance();
    }

    #[test]
    fn peek_test() {
        let source = "12";
        let mut scanner = Scanner::new(source.into());
        let a = scanner.peek();
        let b = scanner.peek();
        scanner.current += 1;
        let c = scanner.peek();
        scanner.current += 1;
        let d = scanner.peek();

        assert_eq!(a, '1');
        assert_eq!(b, '1');
        assert_eq!(c, '2');
        assert_eq!(d, '\0');
    }

    #[test]
    #[should_panic]
    fn invalid_peek() {
        let source = "1";
        let mut scanner = Scanner::new(source.into());
        scanner.current = 2;
        scanner.peek();
    }

    #[test]
    fn peek_next_test() {
        let source = "12";
        let mut scanner = Scanner::new(source.into());
        let a = scanner.peek_next();
        let b = scanner.peek_next();
        scanner.current += 1;
        let c = scanner.peek_next();

        assert_eq!(a, '2');
        assert_eq!(b, '2');
        assert_eq!(c, '\0');
    }

    #[test]
    #[should_panic]
    fn invalid_peek_next() {
        let source = "1";
        let mut scanner = Scanner::new(source.into());
        scanner.current = 1;
        scanner.peek_next();
    }
}
