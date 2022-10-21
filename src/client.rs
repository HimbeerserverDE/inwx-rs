use crate::{call, response};
use crate::{Error, Result};

use std::sync::Arc;

use reqwest::{blocking, Url};

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

impl Into<Url> for Endpoint {
    fn into(self) -> Url {
        Url::parse(self.into()).unwrap()
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
        let transport = self.inner.http.post(self.inner.endpoint.into());

        let request = xmlrpc::Request::new(call.method_name());
        request.arg(call);

        let raw = request.call(transport)?;
        match raw {
            xmlrpc::Value::Struct(map) => {
                let code = map.get("code")
                    .ok_or(Error::Inexistent("code"))?;

                match code {
                    xmlrpc::Value::Int(code) => {
                        if call.expected().contains(code) {
                            let data = map.get("resData")
                                .ok_or(Error::Inexistent("resData"))?;

                            match data {
                                xmlrpc::Value::Struct(response) => {
                                    Ok(response::Response {
                                        status: *code,
                                        data: response,
                                    })
                                },
                                _ => Err(Error::Type("resData", "Struct")),
                            }
                        } else {
                            Err(Error::BadStatus(call.expected(), code))
                        }
                    },
                    _ => Err(Error::Type("code", "Int")),
                }
            },
            _ => Err(Error::BadResponse(raw)),
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.call(call::account::Logout);
    }
}

// The underlying data of a `Client`.
struct ClientRef {
    http: blocking::Client,
    endpoint: Endpoint,
}
