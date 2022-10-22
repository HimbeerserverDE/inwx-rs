// A call to the API.
pub trait Call: Into<xmlrpc::Value> {
    fn method_name(&self) -> &'static str;
    fn expected(&self) -> &'static [i32];
}

pub mod account;
pub mod nameserver;
