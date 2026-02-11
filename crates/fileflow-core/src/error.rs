use std::{fmt, io};

pub type Result<T> = std::result::Result<T, FileFlowError>;

#[derive(Debug)]
pub enum FileFlowError {
    Io(io::Error),
    Message(String),
    Cancelled,
}

impl fmt::Display for FileFlowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileFlowError::Io(e) => write!(f, "I/O error: {e}"),
            FileFlowError::Message(msg) => write!(f, "{msg}"),
            FileFlowError::Cancelled => write!(f, "Job cancelled"),
        }
    }
}

impl std::error::Error for FileFlowError {}

impl From<io::Error> for FileFlowError {
    fn from(value: io::Error) -> Self {
        FileFlowError::Io(value)
    }
}
