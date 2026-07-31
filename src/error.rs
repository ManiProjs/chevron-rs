use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ChevronError {
    Cancelled,
    Validation(String),
    Io(io::Error),
}

impl fmt::Display for ChevronError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cancelled => write!(f, "prompt cancelled"),
            Self::Io(err) => write!(f, "{}", err),
            Self::Validation(message) => write!(f, "validation failed: {message}"),
        }
    }
}

impl std::error::Error for ChevronError {}

impl From<io::Error> for ChevronError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}
