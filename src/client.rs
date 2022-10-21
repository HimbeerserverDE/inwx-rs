use crate::{Error, Result};

use std::sync::Arc;

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

/// A synchronous client to make API calls with.
/// You do **not** need to wrap it in an `Arc` or `Rc`
/// because it already uses an `Arc` internally.
/// [`Rc`]: std::rc::Rc
pub struct Client {
    inner: Arc<ClientRef>,
}

impl Client {
    /// Initialises a session and returns a `Client` if successful.
    pub fn login(ep: Endpoint, user: &str, pass: &str) -> Result<Client> {
        let client = Client {
            inner: Arc::new(ClientRef {
                http: reqwest::Client::builder().cookie_store(true).build()?,
            }),
        };

        client.call(crate::call::account::Login {
            user,
            pass,
            case_insensitive: false,
        })?;

        Ok(client)
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.call(crate::call::account::Logout);
    }
}

// The underlying data of a `Client`.
struct ClientRef {
    http: reqwest::Client,
}
