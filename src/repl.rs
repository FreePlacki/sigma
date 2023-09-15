use colored::Colorize;
use rustyline::{config::Configurer, Editor};

use crate::{
    error::Error,
    interpreter::{self, Environment},
    parser, scanner,
};

pub fn run(source: String, environment: Environment, is_repl: bool) -> Result<Environment, Error> {
    let mut scanner = scanner::Scanner::new(source);
    let tokens = scanner.scan()?;
    let mut parser = parser::Parser::new(tokens.to_owned());
    let expressions = parser.parse()?;
    let mut interpreter = interpreter::Interpreter::new(expressions.to_owned(), environment);

    interpreter.interpret(is_repl)
}

pub fn run_prompt() {
    println!("Sigma {}", env!("CARGO_PKG_VERSION").bright_black().bold());

    let mut rl = Editor::<()>::new().unwrap(); // TODO: add helper
    rl.set_max_history_size(69);
    rl.load_history("history.txt").ok();

    let prompt = format!("{} ", "Σ ❯❯".blue().bold());

    let mut environment = Environment::new();
    loop {
        let source = rl.readline(&prompt);

        match source {
            Ok(src) => {
                if src.is_empty() {
                    continue;
                }
                match run(src.clone(), environment.clone(), true) {
                    Ok(en) => environment = en,
                    Err(er) => er.print_error(&src),
                }
                rl.add_history_entry(src);
            }
            Err(_) => break,
        }
    }
    rl.save_history("history.txt").unwrap();
}
