use std::{fmt, io};

#[derive(Debug, Clone)] // derive std::fmt::Debug on AppError
pub struct GenerationError {
    pub(crate) kind: String,
    pub(crate) message: String,
}

impl fmt::Display for GenerationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "GenerationError {{{}: message: {} }}",
            self.kind, self.message
        )
    }
}

impl From<io::Error> for GenerationError {
    fn from(error: io::Error) -> Self {
        GenerationError {
            kind: String::from("io"),
            message: error.to_string(),
        }
    }
}

impl From<&str> for GenerationError {
    fn from(error: &str) -> Self {
        GenerationError {
            kind: String::from("string"),
            message: error.to_string(),
        }
    }
}