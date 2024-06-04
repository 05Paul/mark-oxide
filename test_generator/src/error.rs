use std::fmt::{Display, Formatter};
use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    SerdeJson(serde_json::Error),
    Reqwest(reqwest::Error),
    Other(String),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::SerdeJson(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::Reqwest(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(error) => write!(f, "{error}"),
            Error::SerdeJson(error) => write!(f, "{error}"),
            Error::Reqwest(error) => write!(f, "{error}"),
            Error::Other(msg) => write!(f, "{msg}"),
        }
    }
}