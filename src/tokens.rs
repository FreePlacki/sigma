#[derive(Debug)]
pub struct Token {
    pub kind: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub start: usize,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    Minus,
    Plus,
    Slash,
    Star,
    Bang,
    Caret,
    Equals,

    // Literals.
    Identifier,
    Number,

    // other
    Error,
    Eof,
}
