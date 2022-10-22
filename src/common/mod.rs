use crate::{Error, Result};

use std::collections::BTreeMap;

use iso8601::DateTime;

pub(crate) fn get_str(map: &BTreeMap<String, xmlrpc::Value>, key: &'static str) -> Result<String> {
    let value = map
        .get(key)
        .ok_or(Error::Inexistent(key))?
        .as_str()
        .ok_or_else(|| Error::Type(key, "String", map.get(key).unwrap().clone()))?;

    Ok(value.to_owned())
}

pub(crate) fn get_i32(map: &BTreeMap<String, xmlrpc::Value>, key: &'static str) -> Result<i32> {
    let value = map
        .get(key)
        .ok_or(Error::Inexistent(key))?
        .as_i32()
        .ok_or_else(|| Error::Type(key, "Int", map.get(key).unwrap().clone()))?;

    Ok(value)
}

pub(crate) fn get_bool(map: &BTreeMap<String, xmlrpc::Value>, key: &'static str) -> Result<bool> {
    let value = map
        .get(key)
        .ok_or(Error::Inexistent(key))?
        .as_bool()
        .ok_or_else(|| Error::Type(key, "Bool", map.get(key).unwrap().clone()))?;

    Ok(value)
}

pub(crate) fn get_datetime(
    map: &BTreeMap<String, xmlrpc::Value>,
    key: &'static str,
) -> Result<DateTime> {
    let value = map
        .get(key)
        .ok_or(Error::Inexistent(key))?
        .as_datetime()
        .ok_or_else(|| Error::Type(key, "DateTime", map.get(key).unwrap().clone()))?;

    Ok(value)
}

pub(crate) fn get_array(
    map: &BTreeMap<String, xmlrpc::Value>,
    key: &'static str,
) -> Result<Vec<xmlrpc::Value>> {
    let value = map
        .get(key)
        .ok_or(Error::Inexistent(key))?
        .as_array()
        .ok_or_else(|| Error::Type(key, "Array", map.get(key).unwrap().clone()))?;

    Ok(value.to_vec())
}

pub(crate) fn get_map(
    map: &BTreeMap<String, xmlrpc::Value>,
    key: &'static str,
) -> Result<BTreeMap<String, xmlrpc::Value>> {
    let value = map
        .get(key)
        .ok_or(Error::Inexistent(key))?
        .as_struct()
        .ok_or_else(|| Error::Type(key, "Struct", map.get(key).unwrap().clone()))?;

    Ok(value.to_owned())
}

pub mod nameserver;
