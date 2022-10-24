// A call to the API.
pub trait Call: Clone + std::fmt::Debug + Into<xmlrpc::Value> {
    fn method_name(&self) -> String;
    fn expected(&self) -> Vec<i32>;
}

pub mod account;
pub mod nameserver;
