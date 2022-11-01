use crate::{call, response};
use crate::{Error, Result};

use std::collections::BTreeMap;
use std::sync::Arc;

use reqwest::{blocking, Url};

/// The INWX environment to use. The Sandbox is good for testing
/// or debugging purposes.
#[derive(Clone, Copy, Debug)]
pub enum Endpoint {
    Production,
    Sandbox,
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
                http: blocking::Client::builder().cookie_store(true).build()?,
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
    pub fn call(&self, call: impl call::Call) -> Result<response::Response> {
        let expected = call.expected();

        let transport = self.inner.http.post::<Url>(self.inner.endpoint.into());

        let binding = call.method_name();
        let request = xmlrpc::Request::new(&binding).arg(call);

        let raw = request.call(transport)?;
        match raw {
            xmlrpc::Value::Struct(map) => {
                let code = map
                    .get("code")
                    .ok_or_else(|| Error::Inexistent("code".into()))?;

                match code {
                    xmlrpc::Value::Int(code) => {
                        if expected.contains(code) {
                            let default = &xmlrpc::Value::Struct(BTreeMap::new());
                            let data = map.get("resData").unwrap_or(default);

                            match data {
                                xmlrpc::Value::Struct(response) => Ok(response::Response {
                                    status: *code,
                                    data: response.clone(),
                                }),
                                _ => Err(Error::Type(
                                    "resData".into(),
                                    "Struct".into(),
                                    data.clone(),
                                )),
                            }
                        } else {
                            Err(Error::BadStatus(expected, *code))
                        }
                    }
                    _ => Err(Error::Type("code".into(), "Int".into(), code.clone())),
                }
            }
            _ => Err(Error::BadResponse(raw.clone())),
        }
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
