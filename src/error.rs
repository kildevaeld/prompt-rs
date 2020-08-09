use std::error::Error as StdError;
use std::fmt;
use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    NoMoreInput,
    UserAborted,
    InvalidChoice(usize),
    Format(fmt::Error),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<fmt::Error> for Error {
    fn from(error: fmt::Error) -> Self {
        Self::Format(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError(err) => write!(f, "Io error: {}", err),
            Error::NoMoreInput => write!(f, "No more input"),
            Error::UserAborted => write!(f, "User aborted"),
            Error::InvalidChoice(idx) => write!(f, "Invalid choice at inedx: {}", idx),
            Error::Format(err) => write!(f, "Formatting error: {}", err),
        }
    }
}

impl StdError for Error {}
