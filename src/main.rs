mod error;
mod expr;
mod interpreter;
mod parser;
mod scanner;
mod tokens;

use colored::Colorize;
use error::Error;
use std::io::Write;

fn run(source: String) -> Result<(), Error> {
    let mut scanner = scanner::Scanner::new(source);
    let tokens = scanner.scan()?;
    let mut parser = parser::Parser::new(tokens.to_owned());
    let expressions = parser.parse()?;
    let interpreter = interpreter::Interpreter::new(expressions.to_owned());
    interpreter.interpret()?;

    Ok(())
}

fn run_prompt() {
    println!("Sigma {}", env!("CARGO_PKG_VERSION").bright_black().bold());

    loop {
        print!("{} ", "Σ ❯❯".blue().bold());
        std::io::stdout().flush().unwrap();

        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        if let Err(e) = run(buffer.clone()) {
            e.print_error(&buffer);
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        2 => {
            let contents = std::fs::read_to_string(&args[1]);
            if let Ok(contents) = contents {
                if let Err(e) = run(contents.clone()) {
                    e.print_error(&contents);
                }
            } else {
                eprintln!("Failed to read file '{}'", args[1]);
            }
        }
        1 => {
            run_prompt();
        }
        _ => eprintln!("Usage: {} <file>", args[0]),
    }
}
