use crate::error::{Error, ErrorKind};
use crate::tokens::{Token, TokenKind};

pub struct Scanner {
    source: String,
    current: usize, // incremented with every char
    start: usize,   // start = current when scanning new token
    pos: usize,     // index in the current line
    line: usize,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            pos: 0,
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
            pos: self.pos,
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
                    pos: self.pos - 1,
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
                    pos: self.pos - 1,
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
                        pos: self.pos - 1,
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
                self.pos = 0;
            }

            '\t' => add_token!(TokenKind::Tab),
            ' ' | '\r' => {}
            '\n' => {
                self.line += 1;
                self.pos = 0
            }

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
                    pos: self.pos - 1, // -1 cause we advanced earlier
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
        self.pos += 1;
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

    macro_rules! test_token {
        ($source:expr; $kind:ident, $line:expr, $pos:expr, $lexeme:expr) => {
            let mut scanner = Scanner::new($source.into());
            scanner.scan().ok();

            assert_eq!(scanner.tokens[0].kind, TokenKind::$kind, "kind");
            assert_eq!(scanner.tokens[0].line, $line, "line");
            assert_eq!(scanner.tokens[0].pos, $pos, "pos");
            assert_eq!(scanner.tokens[0].lexeme, $lexeme, "lexeme");
        };
    }

    #[test]
    fn single_char() {
        test_token!("*"; Star, 0, 0, "*");
    }

    #[test]
    fn comment() {
        test_token!("# abc 123 ~ a\na"; Identifier, 1, 0, "a");
    }

    #[test]
    fn number() {
        test_token!("123"; Number, 0, 2, "123");
        test_token!("1.3121"; Number, 0, 5, "1.3121");
        test_token!("3_321"; Number, 0, 4, "3_321");
        test_token!("3_3_21.3"; Number, 0, 7, "3_3_21.3");
        test_token!(".33_2"; Number, 0, 4, ".33_2");
    }

    #[test]
    #[should_panic]
    fn invalid_number() {
        // TODO implement Error token instead of panic
        test_token!("23.312.1"; Number, 0, 2, "");
    }

    #[test]
    fn identifier() {
        test_token!("abc"; Identifier, 0, 2, "abc");
        test_token!("a_bc"; Identifier, 0, 3, "a_bc");
    }
}
