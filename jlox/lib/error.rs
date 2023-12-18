use std::error::Error;
use std::fmt::Display;
use std::process::{self, ExitCode, Termination};

#[derive(Debug)]
pub enum LoxError {
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

impl From<std::io::Error> for LoxError {
    fn from(value: std::io::Error) -> Self {
        LoxError::IoError
    }
}
