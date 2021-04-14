use std::{fmt, io};
use std::io::ErrorKind;

#[derive(Debug, Clone)] // derive std::fmt::Debug on AppError
pub struct GenerationError {
    pub(crate) kind: String,
    pub(crate) io_subkind: Option<ErrorKind>,
    pub(crate) message: String,
}

impl GenerationError{
    pub fn new(kind: String, message: String) -> GenerationError {
        GenerationError { kind: kind, io_subkind: None, message: message }
    }
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
            io_subkind: Option::from(error.kind()),
            message: error.to_string(),
        }
    }
}

impl From<&str> for GenerationError {
    fn from(error: &str) -> Self {
        GenerationError {
            kind: String::from("string"),
            io_subkind: None,
            message: error.to_string(),
        }
    }
}