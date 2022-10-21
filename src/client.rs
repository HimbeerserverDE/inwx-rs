use crate::{Error, Result};

use reqwest::Url;

/// The INWX environment to use. The Sandbox is good for testing
/// or debugging purposes.
pub enum Endpoint {
    Production,
    Sandbox,
}

impl From<Endpoint> for &str {
    fn from(endpoint: Endpoint) -> &'static str {
        match endpoint {
            Endpoint::Production => "https://api.domrobot.com/xmlrpc/",
            Endpoint::Sandbox => "https://api.ote.domrobot.com/xmlrpc/",
        }
    }
}

impl TryInto<Url> for Endpoint {
    type Error = Error;
    fn try_into(self) -> Result<Url> {
        let url = Url::parse(self.into())?;
        Ok(url)
    }
}
