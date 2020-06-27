use failure::Fail;
use std::fmt;
use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "IO error: {}", _0)]
    IoError(io::Error),
    #[fail(display = "No more input")]
    NoMoreInput,
    #[fail(display = "User aborted")]
    UserAborted,
    #[fail(display = "Invalid Choice: {}", _0)]
    InvalidChoice(usize),
    #[fail(display = "Format error: {}", _0)]
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

/*
error_chain! {

    foreign_links {
        Io(io::Error);
    }

    errors {
        NoMoreInput {
            description("")
            display("")
        }

        UserAborted {
            description("")
            display("")
        }

        InvalidChoice(index:usize) {
            description("")
            display("")
        }
    }

}
*/
