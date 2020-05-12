use serde_json;
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum SkError {
    Default(String),
    Json(serde_json::Error),
    JsonError(String),
    Io(io::Error),
    Http(reqwest::Error),
    BadRequest(String),
}

impl fmt::Display for SkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SkError::Default(ref err) => write!(f, "Generic error: {}", err),
            SkError::Json(ref err) => write!(f, "JSON error: {}", err),
            SkError::Io(ref err) => write!(f, "IO error: {}", err),
            SkError::Http(ref err) => write!(f, "Http error: {}", err),
            SkError::JsonError(ref err) => write!(f, "Http error: {}", err),
            SkError::BadRequest(ref err) => write!(f, "Http error: {}", err),
        }
    }
}

impl From<reqwest::Error> for SkError {
    fn from(err: reqwest::Error) -> SkError {
        SkError::Http(err)
    }
}

impl From<serde_json::Error> for SkError {
    fn from(err: serde_json::Error) -> SkError {
        SkError::Json(err)
    }
}

impl From<io::Error> for SkError {
    fn from(err: io::Error) -> SkError {
        SkError::Io(err)
    }
}

impl error::Error for SkError {
    fn description(&self) -> &str {
        match *self {
            SkError::Default(ref err) => err,
            SkError::Json(ref err) => err.description(),
            SkError::Io(ref err) => err.description(),
            SkError::Http(ref err) => err.description(),
            SkError::JsonError(ref err) => err,
            SkError::BadRequest(ref err) => err,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SkError::Json(ref err) => Some(err),
            SkError::Io(ref err) => Some(err),
            SkError::Http(ref err) => Some(err),
            _ => None,
        }
    }
}
