mod error;
mod token;
pub use crate::error::LoxError;
use std::{io::Write, path::Path};

pub fn run(input: &str) -> Result<(), LoxError> {
    let scanner = token::Scanner::new(input);
    let tokens = scanner.tokens();
    Ok(())
}

fn run_file(input: &String) -> Result<(), LoxError> {
    let path = Path::new(input);
    match path.is_file() && path.exists() {
        true => {
            let content = std::fs::read_to_string(input)?;
            run(&content)
        }
        false => Err(LoxError::IoError),
    }
}

fn run_prompt() -> Result<(), LoxError> {
    let mut buffer = String::new();
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut buffer)?;
        run(&buffer)?;
        buffer.clear();
    }
    Ok(())
}

pub fn main_impl() -> Result<(), LoxError> {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => run_prompt(),
        2 if args[1] != "--help" => run_file(&args[1]),
        _ => {
            println!("Usage: jlox-rs [script]");
            std::process::exit(64)
        }
    }
}
