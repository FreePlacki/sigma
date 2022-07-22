mod scanner;
mod tokens;

use colored::Colorize;
use std::io::Write;

fn run(source: String) {
    let mut scanner = scanner::Scanner::new(source);
    let tokens = scanner.scan();
    println!("{:?}", tokens);
}

fn run_prompt() {
    println!("Sigma version {}", env!("CARGO_PKG_VERSION").bold());

    loop {
        print!("{} ", "Σ".green());
        std::io::stdout().flush().unwrap();

        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        run(buffer);
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        2 => {
            let contents = std::fs::read_to_string(&args[1]);
            if let Ok(contents) = contents {
                run(contents);
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
