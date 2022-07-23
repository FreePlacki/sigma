use colored::Colorize;

pub enum ErrorType {
    UnexpectedCharacter,
}

fn message(err: ErrorType) -> &'static str {
    match err {
        ErrorType::UnexpectedCharacter => "Unexpected character",
    }
}

pub fn error(source: &str, line: usize, pos: usize, error_type: ErrorType) {
    eprintln!(
        "{} [{}:{}] {}",
        "Error".red().bold(),
        line + 1,
        pos,
        message(error_type).bold()
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
        " ".repeat(pos + 1),
        "^".red().bold()
    );
}
