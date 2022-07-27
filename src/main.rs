mod error;
mod expr;
mod interpreter;
mod parser;
mod repl;
mod scanner;
mod tokens;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        2 => {
            let contents = std::fs::read_to_string(&args[1]);
            if let Ok(contents) = contents {
                if let Err(e) = repl::run(contents.clone()) {
                    e.print_error(&contents);
                }
            } else {
                eprintln!("Failed to read file '{}'", args[1]);
            }
        }
        1 => {
            repl::run_prompt();
        }
        _ => eprintln!("Usage: {} <file>", args[0]),
    }
}
