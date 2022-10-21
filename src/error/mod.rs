use std::fmt;

/// The Errors that may occur around Clients.
#[derive(Debug)]
pub enum Error {
    ParseUrl(url::ParseError),
    Reqwest(reqwest::Error),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseUrl(err) => write!(fmt, "can't parse Url: {}", err),
            Error::Reqwest(err) => write!(fmt, "reqwest error: {}", err),
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

/// A `Result` alias where the `Err` case is `inwx::Error`.
pub type Result<T> = std::result::Result<T, Error>;
