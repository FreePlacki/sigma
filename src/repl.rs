use colored::Colorize;
use rustyline::{config::Configurer, Editor};

use crate::{error::Error, interpreter, parser, scanner};

pub fn run(source: String) -> Result<(), Error> {
    let mut scanner = scanner::Scanner::new(source);
    let tokens = scanner.scan()?;
    let mut parser = parser::Parser::new(tokens.to_owned());
    let expressions = parser.parse()?;
    let interpreter = interpreter::Interpreter::new(expressions.to_owned());
    interpreter.interpret()?;

    Ok(())
}

pub fn run_prompt() {
    println!("Sigma {}", env!("CARGO_PKG_VERSION").bright_black().bold());

    let mut rl = Editor::<()>::new().unwrap(); // TODO add helper
    rl.set_max_history_size(69);
    rl.load_history("history.txt").unwrap();

    let prompt = format!("{} ", "Σ ❯❯".blue().bold());
    loop {
        let source = rl.readline(&prompt);

        match source {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }
                if let Err(e) = run(line.clone()) {
                    e.print_error(&line);
                } else {
                    rl.add_history_entry(line);
                }
            }
            Err(_) => break,
        }
    }
    rl.save_history("history.txt").unwrap();
}
