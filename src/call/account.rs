use super::*;

use serde_derive::{Deserialize, Serialize};

// Contains login information. Used to create an API session.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct Login {
    pub(crate) user: String,
    pub(crate) pass: String,
    #[serde(rename = "case-insensitive")]
    pub(crate) case_insensitive: bool,
}

impl Call for Login {
    fn method_name(&self) -> String {
        String::from("account.login")
    }

    fn expected(&self) -> Vec<i32> {
        vec![1000]
    }
}

impl Response<crate::response::account::Login> for Login {}

// Contains no information. This just signals to the server
// that it should end the session.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct Logout;

impl Call for Logout {
    fn method_name(&self) -> String {
        String::from("account.logout")
    }

    fn expected(&self) -> Vec<i32> {
        vec![1500]
    }
}

impl Response<()> for Logout {}
