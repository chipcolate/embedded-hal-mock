use std::{error::Error as StdError, fmt, io};

/// Errors that may occur during mocking.
#[derive(PartialEq, Clone, Debug)]
pub enum MockError {
    /// An I/O-Error occurred
    Io(io::ErrorKind),
    /// Generic error message
    Generic(String),
}

impl From<io::Error> for MockError {
    fn from(e: io::Error) -> Self {
        MockError::Io(e.kind())
    }
}

impl fmt::Display for MockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MockError::Io(kind) => write!(f, "I/O error: {:?}", kind),
            MockError::Generic(msg) => write!(f, "Generic error: {:?}", msg),
        }
    }
}

impl StdError for MockError {}
