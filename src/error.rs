use colored::Colorize;

pub enum ErrorKind {
    UnexpectedCharacter,
    ExpectedExpression,
    MissingRightParen,
    DivisionByZero,
    FactorialDomain,
    InvalidAssignment,
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
            ErrorKind::DivisionByZero => "Division by zero!",
            ErrorKind::FactorialDomain => "Factorial is only defined for natural numbers",
            ErrorKind::InvalidAssignment => "Can only assign values to variables",
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
