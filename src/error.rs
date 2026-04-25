use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum AkError {
    Io(std::io::Error),
    Validation(&'static str),
}

impl Display for AkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "I/O error: {err}"),
            Self::Validation(msg) => write!(f, "validation error: {msg}"),
        }
    }
}

impl Error for AkError {}

impl From<std::io::Error> for AkError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

pub type AkResult<T> = Result<T, AkError>;
