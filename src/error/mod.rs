use std::fmt;

/// The Errors that may occur around Clients.
#[derive(Debug)]
pub enum Error {
    ParseUrl(url::ParseError),
    Reqwest(reqwest::Error),
    XmlRpc(xmlrpc::Error),
    Inexistent(&'static str),
    Type(&'static str, &'static str, xmlrpc::Value),
    BadResponse(xmlrpc::Value),
    BadStatus(&'static [i32], i32),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseUrl(err) => write!(fmt, "can't parse Url: {}", err),
            Error::Reqwest(err) => write!(fmt, "reqwest error: {}", err),
            Error::XmlRpc(err) => write!(fmt, "xmlrpc error: {}", err),
            Error::Inexistent(what) => {
                write!(fmt, "parameter {} does not exist", what)
            }
            Error::Type(what, exp, got) => {
                write!(
                    fmt,
                    "parameter {what} is of wrong type {got:?} (expected: {exp})"
                )
            }
            Error::BadResponse(resp) => write!(fmt, "bad response: {:?}", resp),
            Error::BadStatus(expected, got) => {
                write!(fmt, "bad status {} (expected: {:?}", got, expected)
            }
        }
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Self::ParseUrl(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}

impl From<xmlrpc::Error> for Error {
    fn from(err: xmlrpc::Error) -> Self {
        Self::XmlRpc(err)
    }
}

/// A `Result` alias where the `Err` case is `inwx::Error`.
pub type Result<T> = std::result::Result<T, Error>;
