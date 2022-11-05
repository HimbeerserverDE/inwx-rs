use super::Response;

use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct Login {
    #[serde(rename = "customerId")]
    pub customer_id: i32,
    #[serde(rename = "customerNo")]
    pub customer_number: i32,
    #[serde(rename = "accountId")]
    pub account_id: i32,
    pub tfa: String,
}

impl Response for Login {
    fn unwrap(wrapped: Option<Self>) -> Self {
        wrapped.unwrap()
    }
}
