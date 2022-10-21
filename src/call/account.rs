use std::collections::BTreeMap;

// Contains login information. Used to create an API session.
pub(crate) struct Login<'a> {
    pub(crate) user: &'a str,
    pub(crate) pass: &'a str,
    pub(crate) case_insensitive: bool,
}

impl From<Login<'_>> for xmlrpc::Value {
    fn from(login: Login<'_>) -> Self {
        let mut map = BTreeMap::new();

        map.insert("user".into(), login.user.into());
        map.insert("pass".into(), login.pass.into());
        map.insert("case-insensitive".into(), login.case_insensitive.into());

        xmlrpc::Value::Struct(map)
    }
}
