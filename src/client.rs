use crate::call;
use crate::response::{self, ResponseData};
use crate::{Error, Result};

use std::net::Ipv4Addr;
use std::sync::Arc;

use reqwest::{blocking, Url};

/// The INWX environment to use. The Sandbox is good for testing
/// or debugging purposes.
#[derive(Clone, Copy, Debug)]
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

impl From<Endpoint> for String {
    fn from(endpoint: Endpoint) -> String {
        match endpoint {
            Endpoint::Production => String::from("https://api.domrobot.com/xmlrpc/"),
            Endpoint::Sandbox => String::from("https://api.ote.domrobot.com/xmlrpc/"),
        }
    }
}

impl From<Endpoint> for Url {
    fn from(endpoint: Endpoint) -> Self {
        String::from(endpoint).parse().unwrap()
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
    pub fn login(ep: Endpoint, user: String, pass: String) -> Result<Client> {
        let client = Client {
            inner: Arc::new(ClientRef {
                http: blocking::Client::builder()
                    .cookie_store(true)
                    .resolve(
                        "api.domrobot.com",
                        (Ipv4Addr::new(185, 181, 104, 26), 443).into(),
                    )
                    .resolve(
                        "api.ote.domrobot.com",
                        (Ipv4Addr::new(185, 181, 104, 52), 443).into(),
                    )
                    .build()?,
                endpoint: ep,
            }),
        };

        client.call(call::account::Login {
            user,
            pass,
            case_insensitive: false,
        })?;

        Ok(client)
    }

    /// Issues a `Call` and returns a `Response`
    /// if successful and if the status code
    /// matches one of the expected status codes.
    pub fn call<T, U>(&self, call: T) -> Result<U>
    where
        T: call::Call + call::Response<U>,
        U: response::Response + Clone + serde::de::DeserializeOwned,
    {
        let expected = call.expected();
        let xml = serde_xmlrpc::request_to_str(&call.method_name(), vec![call])?;

        let raw_response = self
            .inner
            .http
            .post::<Url>(self.inner.endpoint.into())
            .body(xml)
            .send()?
            .text()?;

        let resp: ResponseData<U> = serde_xmlrpc::response_from_str(&raw_response)?;

        if !expected.contains(&resp.status) {
            return Err(Error::BadStatus(expected, resp.status));
        }

        Ok(U::unwrap(resp.params))
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        // Ignore the result. Failed logout doesn't really matter.
        self.call(call::account::Logout).ok();
    }
}

// The underlying data of a `Client`.
struct ClientRef {
    http: blocking::Client,
    endpoint: Endpoint,
}
