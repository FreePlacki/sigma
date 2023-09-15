use colored::Colorize;

pub enum ErrorKind {
    UnexpectedCharacter,
    ExpectedExpression,
    MissingRightParen,
    MissingRightBracket,
    DivisionByZero,
    FactorialDomain,
    FactorialDimension,
    InvalidAssignment,
    InvalidUnitsAdd,
    InvalidUnitsSub,
    InvalidUnitsPow,
    UndefinedVariable,
}

pub struct Error {
    pub line: usize,
    pub pos: usize,
    pub kind: ErrorKind,
}

impl Error {
    fn message(&self, err: &ErrorKind) -> &'static str {
        match err {
            ErrorKind::UnexpectedCharacter => "Unexpected character",
            ErrorKind::ExpectedExpression => "Unable to parse expression",
            ErrorKind::MissingRightParen => "Expected ')' after opening '('",
            ErrorKind::MissingRightBracket => "Expected ']' after opening '['",
            ErrorKind::DivisionByZero => "Division by zero!",
            ErrorKind::FactorialDomain => "Factorial is only defined for natural numbers",
            ErrorKind::FactorialDimension => "Can only take factorial of dimensionless values",
            ErrorKind::InvalidAssignment => "Can only assign values to variables",
            ErrorKind::InvalidUnitsAdd => "Cannot add values with different units",
            ErrorKind::InvalidUnitsSub => "Cannot subtract values with different units",
            ErrorKind::InvalidUnitsPow => "Can only raise to a power of dimensionless values",
            ErrorKind::UndefinedVariable => "Undefined variable",
        }
    }

    pub fn print_error(&self, source: &str) {
        let line = std::cmp::min(source.lines().count() - 1, self.line);
        eprintln!(
            "{} [{}:{}] {}",
            "Error".red().bold(),
            line + 1,
            self.pos + 1,
            self.message(&self.kind).bold()
        );
        println!("{:>6}", "|".blue().bold());
        println!(
            "{:>4} {}\t{}",
            line + 1,
            "|".blue().bold(),
            source.lines().nth(line).unwrap()
        );
        println!(
            "{:>6}{}{}",
            "|".blue().bold(),
            " ".repeat(self.pos + 2),
            "^".red().bold()
        );
    }
}
