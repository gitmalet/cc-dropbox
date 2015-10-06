use std::error;
use std::fmt;
use std::io;
use hyper;
use serde_json;

#[derive(Debug)]
pub enum DropBoxError {
    APIError(String),
    HyperError(hyper::error::Error),
    IOError(io::Error),
    SerdeError(serde_json::error::Error),
}

impl fmt::Display for DropBoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DropBoxError::HyperError(ref err) => err.fmt(f),
            DropBoxError::IOError(ref err) => err.fmt(f),
            DropBoxError::SerdeError(ref err) => err.fmt(f),
            DropBoxError::APIError(ref res) => write!(f, "JSON Response: {}", res),
        }
    }
}

impl error::Error for DropBoxError {
    fn description(&self) -> &str {
        match *self {
            DropBoxError::HyperError(ref err) => err.description(),
            DropBoxError::IOError(ref err) => err.description(),
            DropBoxError::SerdeError(ref err) => err.description(),
            DropBoxError::APIError(ref res) => "Dropbox could not process request",
        }
    }
}

impl From<io::Error> for DropBoxError {
    fn from(error: io::Error) -> DropBoxError {
        DropBoxError::IOError(error)
    }
}

impl From<hyper::error::Error> for DropBoxError {
    fn from(error: hyper::error::Error) -> DropBoxError {
        DropBoxError::HyperError(error)
    }
}

impl From<serde_json::error::Error> for DropBoxError {
    fn from(error: serde_json::error::Error) -> DropBoxError {
        DropBoxError::SerdeError(error)
    }
}

