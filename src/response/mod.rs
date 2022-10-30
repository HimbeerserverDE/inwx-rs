use std::collections::BTreeMap;

/// A Response to an API call including status and data.
/// Data is guaranteed to be a `Struct`.
/// [`Struct`]: xmlrpc::Value::Struct
#[derive(Clone, Debug)]
pub struct Response {
    pub status: i32,
    pub(crate) data: BTreeMap<String, xmlrpc::Value>,
}

pub mod nameserver;
