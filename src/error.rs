use std::fmt;

/// The Errors that may occur around Clients.
#[derive(Debug)]
pub enum Error {
    ParseUrl(url::ParseError),
    Reqwest(reqwest::Error),
    XmlRpc(xmlrpc::Error),
    Inexistent(String),
    Type(String, String, xmlrpc::Value),
    BadResponse(xmlrpc::Value),
    BadStatus(Vec<i32>, i32),
    BadVariant(String, String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseUrl(e) => write!(fmt, "can't parse Url: {}", e),
            Error::Reqwest(e) => write!(fmt, "reqwest eor: {}", e),
            Error::XmlRpc(e) => write!(fmt, "xmlrpc eor: {}", e),
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
            Error::BadVariant(enum_name, var) => {
                write!(fmt, "{} is not a valid enum variant for {}", var, enum_name)
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

impl From<xmlrpc::Error> for Error {
    fn from(e: xmlrpc::Error) -> Self {
        Self::XmlRpc(e)
    }
}

/// A `Result` alias where the `Err` case is `inwx::Error`.
pub type Result<T> = std::result::Result<T, Error>;
