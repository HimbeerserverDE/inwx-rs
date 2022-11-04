use std::fmt;

/// The Errors that may occur around Clients.
#[derive(Debug)]
pub enum Error {
    ParseUrl(url::ParseError),
    Reqwest(reqwest::Error),
    SerdeXmlRpc(serde_xmlrpc::Error),
    Inexistent(String),
    MalformedResponse(serde_xmlrpc::Value),
    BadStatus(Vec<i32>, i32),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseUrl(e) => write!(fmt, "can't parse Url: {}", e),
            Error::Reqwest(e) => write!(fmt, "reqwest error: {}", e),
            Error::SerdeXmlRpc(e) => write!(fmt, "serde_xmlrpc error: {}", e),
            Error::Inexistent(what) => {
                write!(fmt, "parameter {} does not exist", what)
            }
            Error::MalformedResponse(resp) => {
                write!(fmt, "malformed response: {:?}", resp)
            }
            Error::BadStatus(expected, got) => {
                write!(fmt, "bad status {} (expected: {:?}", got, expected)
            }
        }
    }
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Self::ParseUrl(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

impl From<serde_xmlrpc::Error> for Error {
    fn from(e: serde_xmlrpc::Error) -> Self {
        Self::SerdeXmlRpc(e)
    }
}

/// A `Result` alias where the `Err` case is `inwx::Error`.
pub type Result<T> = std::result::Result<T, Error>;
