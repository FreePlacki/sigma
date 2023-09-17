use colored::Colorize;

pub enum ErrorKind {
    UnexpectedCharacter,
    ExpectedExpression,
    ExpectedFunctionName,
    MissingRightParen,
    MissingRightBracket,
    MissingComma,
    DivisionByZero,
    FactorialDomain,
    InvalidAssignment,
    InvalidUnitsAdd,
    InvalidUnitsSub,
    InvalidUnitsPow,
    UndefinedVariable,
    UndefinedFunction,
    InvalidNumberOfArgs(String, usize, usize),
    ExpectDimensionless(String),
    InvalidDomain(String),
}

pub struct Error {
    pub line: usize,
    pub pos: usize,
    pub kind: ErrorKind,
}

impl Error {
    fn message(&self, err: &ErrorKind) -> String {
        match err {
            ErrorKind::UnexpectedCharacter => "Unexpected character".into(),
            ErrorKind::ExpectedExpression => "Unable to parse expression".into(),
            ErrorKind::ExpectedFunctionName => "Expected a function name before '('".into(),
            ErrorKind::MissingRightParen => "Expected ')' after opening '('".into(),
            ErrorKind::MissingRightBracket => "Expected ']' after opening '['".into(),
            ErrorKind::MissingComma => "Expected ',' after a function argument".into(),
            ErrorKind::DivisionByZero => "Division by zero!".into(),
            ErrorKind::FactorialDomain => "Factorial is only defined for natural numbers".into(),
            ErrorKind::InvalidAssignment => "Can only assign values to variables".into(),
            ErrorKind::InvalidUnitsAdd => "Cannot add values with different units".into(),
            ErrorKind::InvalidUnitsSub => "Cannot subtract values with different units".into(),
            ErrorKind::InvalidUnitsPow => {
                "Can only raise to a power of dimensionless values".into()
            }
            ErrorKind::UndefinedVariable => "Undefined variable".into(),
            ErrorKind::UndefinedFunction => "Undefined function".into(),
            ErrorKind::InvalidNumberOfArgs(name, expected, given) => {
                format!(
                    "'{name}' takes {expected} argument{} but {given} {} provided",
                    if *expected == 1 { "" } else { "s" },
                    if *given == 0 || *given == 1 {
                        "was"
                    } else {
                        "were"
                    }
                )
            }
            ErrorKind::ExpectDimensionless(name) => {
                format!("Can only take '{name}' of dimensionless values")
            }
            ErrorKind::InvalidDomain(name) => {
                format!("Argument out of the domain of '{name}'")
            }
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
