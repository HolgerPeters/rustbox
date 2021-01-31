use std::fmt;
use std::io;

#[allow(dead_code)]

pub type Result<T> = std::result::Result<T, ProcessingError>;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ProcessingError {
    SourceUnavailable(String),
    DestinationUnavailable(String),
    GeneralIOError(String),
}

impl fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProcessingError::SourceUnavailable(s) => {
                write!(f, "Source {} is not available, consult logs for details", s)
            }
            ProcessingError::DestinationUnavailable(d) => {
                write!(f, "Source {} is not available, consult logs for details", d)
            }
            ProcessingError::GeneralIOError(msg) => {
                write!(f, "IO Error {}", msg)
            }
        }
    }
}

impl From<io::Error> for ProcessingError {
    fn from(error: io::Error) -> Self {
        ProcessingError::GeneralIOError(error.to_string())
    }
}
