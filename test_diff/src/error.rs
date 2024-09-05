use std::fmt::{Display, Formatter};
use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    ROXML(roxmltree::Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<roxmltree::Error> for Error {
    fn from(value: roxmltree::Error) -> Self {
        Error::ROXML(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(error) => write!(f, "{error}"),
            Error::ROXML(error) => write!(f, "{error}"),
        }
    }
}