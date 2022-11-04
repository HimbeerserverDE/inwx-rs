/// A call to the API.
pub trait Call: Clone + std::fmt::Debug + serde::Serialize {
    fn method_name(&self) -> String;
    fn expected(&self) -> Vec<i32>;
}

/// This trait indicates the response data type to this a `Call`.
pub trait Response<T> {}

pub mod account;
pub mod nameserver;
