use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(bound(deserialize = "T: Clone + serde::de::DeserializeOwned"))]
pub struct ResponseData<T> {
    #[serde(rename = "code")]
    pub status: i32,
    #[serde(rename = "resData")]
    pub params: Option<T>,
}

pub trait Response: Sized {
    fn unwrap(_: Option<Self>) -> Self;
}

impl Response for () {
    fn unwrap(_: Option<Self>) -> Self {}
}

pub mod account;
pub mod nameserver;
