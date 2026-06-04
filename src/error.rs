use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum AkError {
    Io(std::io::Error),
    Validation(&'static str),
    Command { status: i32, stderr: String },
    Parse(String),
}

impl Display for AkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "I/O error: {err}"),
            Self::Validation(msg) => write!(f, "validation error: {msg}"),
            Self::Command { status, stderr } => {
                if stderr.is_empty() {
                    write!(f, "command failed with exit code {status}")
                } else {
                    write!(f, "command failed with exit code {status}: {stderr}")
                }
            }
            Self::Parse(msg) => write!(f, "parse error: {msg}"),
        }
    }
}

impl Error for AkError {}

impl From<std::io::Error> for AkError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

#[cfg(feature = "serde")]
impl From<serde_json::Error> for AkError {
    fn from(value: serde_json::Error) -> Self {
        Self::Parse(value.to_string())
    }
}

pub type AkResult<T> = Result<T, AkError>;
