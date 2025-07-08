//! Error handling for the neatify library

use std::fmt;

#[derive(Debug)]
pub enum NeatifyError {
    /// Error when a file is not supported by any formatter
    UnsupportedFile(String),
    /// Error when a file cannot be read or written
    IoError(std::io::Error),
    /// Error when formatting fails
    FormattingError(String),
}

impl fmt::Display for NeatifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NeatifyError::UnsupportedFile(path) => write!(f, "Unsupported file: {}", path),
            NeatifyError::IoError(err) => write!(f, "IO error: {}", err),
            NeatifyError::FormattingError(msg) => write!(f, "Formatting error: {}", msg),
        }
    }
}

impl std::error::Error for NeatifyError {}

impl From<std::io::Error> for NeatifyError {
    fn from(err: std::io::Error) -> Self {
        NeatifyError::IoError(err)
    }
}
