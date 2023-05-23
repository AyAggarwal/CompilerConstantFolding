use crate::Rule;
use pest::error::Error as PestError;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum CompilerError {
    Underflow,
    Overflow,
    DivByZero,
    MismatchType,
}

#[derive(Debug, PartialEq)]
pub enum GenerationError {
    FileReadError,
    FileWriteError,
    CompilerError(CompilerError),
    ParseError(PestError<Rule>),
}

// simple display for error variants related to compilation
impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CompilerError::Underflow => write!(f, "Integer underflow during evaluation"),
            CompilerError::Overflow => write!(f, "Integer overflow during evaluation"),
            CompilerError::DivByZero => write!(f, "Division by Zero during evaluation"),
            CompilerError::MismatchType => {
                write!(f, "Operation on mismatched types during evaluation")
            }
        }
    }
}

impl fmt::Display for GenerationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GenerationError::FileReadError => write!(f, "Error reading file path"),
            GenerationError::FileWriteError => write!(f, "Error writing file to path"),
            GenerationError::CompilerError(e) => write!(f, "{}", e),
            GenerationError::ParseError(e) => write!(f, "{}", e),
        }
    }
}

impl From<PestError<Rule>> for GenerationError {
    fn from(value: PestError<Rule>) -> Self {
        GenerationError::ParseError(value)
    }
}

impl From<CompilerError> for GenerationError {
    fn from(value: CompilerError) -> Self {
        GenerationError::CompilerError(value)
    }
}
