use super::Call;

use std::collections::BTreeMap;

// Contains login information. Used to create an API session.
#[derive(Clone, Debug)]
pub(crate) struct Login {
    pub(crate) user: String,
    pub(crate) pass: String,
    pub(crate) case_insensitive: bool,
}

impl From<Login> for xmlrpc::Value {
    fn from(login: Login) -> Self {
        let mut map = BTreeMap::new();

        map.insert("user".into(), login.user.into());
        map.insert("pass".into(), login.pass.into());
        map.insert("case-insensitive".into(), login.case_insensitive.into());

        xmlrpc::Value::Struct(map)
    }
}

impl Call for Login {
    fn method_name(&self) -> String {
        String::from("account.login")
    }

    fn expected(&self) -> Vec<i32> {
        vec![1000]
    }
}

// Contains no information. This just signals to the server
// that it should end the session.
#[derive(Clone, Debug)]
pub(crate) struct Logout;

impl From<Logout> for xmlrpc::Value {
    fn from(_logout: Logout) -> Self {
        xmlrpc::Value::Nil
    }
}

impl Call for Logout {
    fn method_name(&self) -> String {
        String::from("account.logout")
    }
    fn expected(&self) -> Vec<i32> {
        vec![1500]
    }
}
