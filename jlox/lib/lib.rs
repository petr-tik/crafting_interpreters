mod error;
mod token;
pub use crate::error::LoxError;
use std::io::Write;

fn run_file(_nth: &String) -> Result<(), LoxError> {
    Err(LoxError::IoError)
}

fn run_prompt() -> Result<(), LoxError> {
    let mut buffer = String::new();
    while (true) {
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut buffer);
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
