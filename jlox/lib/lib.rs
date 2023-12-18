mod error;
mod token;
pub use crate::error::LoxError;

fn run_file(_nth: &String) -> Result<(), LoxError> {
    Err(LoxError::IoError)
}

fn run_prompt() -> Result<(), LoxError> {
    Ok(todo!())
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
