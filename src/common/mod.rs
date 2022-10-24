use crate::{Error, Result};

use std::collections::BTreeMap;

use iso8601::DateTime;

pub(crate) fn get_str(map: &BTreeMap<String, xmlrpc::Value>, key: String) -> Result<String> {
    let value = map
        .get(&key)
        .ok_or_else(|| Error::Inexistent(key.clone()))?
        .as_str()
        .ok_or_else(|| Error::Type(key.clone(), "String".into(), map.get(&key).unwrap().clone()))?;

    Ok(value.to_owned())
}

pub(crate) fn get_i32(map: &BTreeMap<String, xmlrpc::Value>, key: String) -> Result<i32> {
    let value = map
        .get(&key)
        .ok_or_else(|| Error::Inexistent(key.clone()))?
        .as_i32()
        .ok_or_else(|| Error::Type(key.clone(), "Int".into(), map.get(&key).unwrap().clone()))?;

    Ok(value)
}

pub(crate) fn get_bool(map: &BTreeMap<String, xmlrpc::Value>, key: String) -> Result<bool> {
    let value = map
        .get(&key)
        .ok_or_else(|| Error::Inexistent(key.clone()))?
        .as_bool()
        .ok_or_else(|| Error::Type(key.clone(), "Bool".into(), map.get(&key).unwrap().clone()))?;

    Ok(value)
}

pub(crate) fn get_datetime(map: &BTreeMap<String, xmlrpc::Value>, key: String) -> Result<DateTime> {
    let value = map
        .get(&key)
        .ok_or_else(|| Error::Inexistent(key.clone()))?
        .as_datetime()
        .ok_or_else(|| {
            Error::Type(
                key.clone(),
                "DateTime".into(),
                map.get(&key).unwrap().clone(),
            )
        })?;

    Ok(value)
}

pub(crate) fn get_array(
    map: &BTreeMap<String, xmlrpc::Value>,
    key: String,
) -> Result<Vec<xmlrpc::Value>> {
    let value = map
        .get(&key)
        .ok_or_else(|| Error::Inexistent(key.clone()))?
        .as_array()
        .ok_or_else(|| Error::Type(key.clone(), "Array".into(), map.get(&key).unwrap().clone()))?;

    Ok(value.to_vec())
}

pub(crate) fn get_map(
    map: &BTreeMap<String, xmlrpc::Value>,
    key: String,
) -> Result<BTreeMap<String, xmlrpc::Value>> {
    let value = map
        .get(&key)
        .ok_or_else(|| Error::Inexistent(key.clone()))?
        .as_struct()
        .ok_or_else(|| Error::Type(key.clone(), "Struct".into(), map.get(&key).unwrap().clone()))?;

    Ok(value.to_owned())
}

pub mod nameserver;
