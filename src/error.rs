use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ChevronError {
    Cancelled,
    Io(io::Error),
}

impl fmt::Display for ChevronError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cancelled => write!(f, "prompt cancelled"),
            Self::Io(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for ChevronError {}

impl From<io::Error> for ChevronError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}
