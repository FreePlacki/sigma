use colored::Colorize;
use rustyline::{config::Configurer, Editor};

use crate::{
    error::Error,
    interpreter::{self, Environment},
    parser, scanner,
};

pub fn run(source: String, environment: Environment, is_repl: bool, filename: String) -> Result<Environment, Error> {
    let mut scanner = scanner::Scanner::new(source);
    let tokens = scanner.scan()?;
    let mut parser = parser::Parser::new(tokens.to_owned());
    let expressions = parser.parse()?;
    let mut interpreter = interpreter::Interpreter::new(expressions.to_owned(), environment);

    interpreter.interpret(is_repl, filename)
}

pub fn run_prompt() {
    println!("Sigma {}", env!("CARGO_PKG_VERSION").bright_black().bold());

    let mut rl = Editor::<()>::new().unwrap(); // TODO: add helper
    rl.set_max_history_size(69);

    let mut sigma_dir = dirs::home_dir().expect("Cannot find home directory");
    sigma_dir.push(".sigma");
    let mut history_dir = sigma_dir.clone();
    history_dir.push("history.txt");
    rl.load_history(&history_dir).ok();

    let prompt = format!("{} ", "Σ ❯❯".blue().bold());

    let mut environment = Environment::new();
    loop {
        let source = rl.readline(&prompt);

        match source {
            Ok(src) => {
                if src.is_empty() {
                    continue;
                }
                match run(src.clone(), environment.clone(), true, "".into()) {
                    Ok(en) => environment = en,
                    Err(er) => er.print_error(&src),
                }
                rl.add_history_entry(src);
            }
            Err(_) => break,
        }
    }
    if let Err(e) = std::fs::create_dir_all(&sigma_dir) {
        eprintln!("Cannot create directory '~/.sigma', {e}");
        return;
    }
    rl.save_history(&history_dir).unwrap();
}
