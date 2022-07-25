use colored::Colorize;

pub enum ErrorKind {
    UnexpectedCharacter,
}

fn message(err: ErrorKind) -> &'static str {
    match err {
        ErrorKind::UnexpectedCharacter => "Unexpected character",
    }
}

pub fn error(source: &str, line: usize, pos: usize, error_kind: ErrorKind) {
    eprintln!(
        "{} [{}:{}] {}",
        "Error".red().bold(),
        line + 1,
        pos,
        message(error_kind).bold()
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
