mod error;
pub use crate::error::LoxError;

fn run_file(nth: &String) -> Result<(), LoxError> {
    Err(LoxError::IoError)
}

fn run_prompt() -> Result<(), LoxError> {
    Ok(todo!())
}

pub fn main_impl() -> Result<(), LoxError> {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        0 => run_prompt(),
        1 => run_file(&args[0]),
        _ => {
            println!("Usage: jlox-rs [script]");
            std::process::exit(64)
        }
    }
}
