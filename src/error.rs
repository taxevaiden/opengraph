use std::error;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum Error {
    Unexpected,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Error::Unexpected => write!(f, "UnexpectedError"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        ""
    }
}
