#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub pos: usize, // position of tokens last char in line
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Tab,
    Minus,
    Plus,
    Slash,
    Star,
    Bang,
    Caret,
    Equals,
    Comma,

    // Literals.
    Identifier,
    Number,
    String,

    // other
    Import,
    Error,
    Eof,
}
