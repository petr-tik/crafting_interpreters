use std::error::Error;
use std::fmt::Display;
use std::process::{self, ExitCode, Termination};

#[derive(Debug)]
enum LoxError {
    IoError,
    ParserError,
    EvaluationError,
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("not done")
    }
}

impl Termination for LoxError {
    fn report(self) -> process::ExitCode {
        match self {
            LoxError::IoError => {
                println!("Couldn't access IO resources");
                return ExitCode::from(1);
            }
            LoxError::ParserError => {
                println!("Failed to parse input");
                return ExitCode::from(2);
            }
            LoxError::EvaluationError => {
                println!("Couldn't evaluate your program");
                return ExitCode::from(3);
            }
        }
    }
}

impl Error for LoxError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

fn main_impl() -> Result<(), LoxError> {
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

fn run_file(nth: &String) -> Result<(), LoxError> {
    Err(LoxError::IoError)
}

fn run_prompt() -> Result<(), LoxError> {
    Ok(todo!())
}

fn main() -> Result<(), LoxError> {
    main_impl()
}
