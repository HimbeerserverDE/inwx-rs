use crate::call::nameserver::RecordType;
use crate::{Error, Result};

use std::collections::BTreeMap;
use std::fmt;

use iso8601::DateTime;

fn get_str(map: &BTreeMap<String, xmlrpc::Value>, key: &'static str) -> Result<String> {
    let value = map
        .get(key)
        .ok_or(Error::Inexistent(key))?
        .as_str()
        .ok_or_else(|| Error::Type(key, "String", map.get(key).unwrap().clone()))?;

    Ok(value.to_owned())
}

fn get_i32(map: &BTreeMap<String, xmlrpc::Value>, key: &'static str) -> Result<i32> {
    let value = map
        .get(key)
        .ok_or(Error::Inexistent(key))?
        .as_i32()
        .ok_or_else(|| Error::Type(key, "Int", map.get(key).unwrap().clone()))?;

    Ok(value)
}

fn get_bool(map: &BTreeMap<String, xmlrpc::Value>, key: &'static str) -> Result<bool> {
    let value = map
        .get(key)
        .ok_or(Error::Inexistent(key))?
        .as_bool()
        .ok_or_else(|| Error::Type(key, "Bool", map.get(key).unwrap().clone()))?;

    Ok(value)
}

/// The domain type. Can be master or slave.
#[derive(Clone, Debug)]
pub enum DomainType {
    Master,
    Slave,
}

impl fmt::Display for DomainType {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(match self {
            Self::Master => "MASTER",
            Self::Slave => "SLAVE",
        })
    }
}

impl TryFrom<xmlrpc::Value> for DomainType {
    type Error = Error;
    fn try_from(v: xmlrpc::Value) -> Result<Self> {
        if let xmlrpc::Value::String(s) = v {
            match s.as_str() {
                "MASTER" => Ok(Self::Master),
                "SLAVE" => Ok(Self::Slave),
                _ => Err(Error::BadVariant("DomainType", s)),
            }
        } else {
            Err(Error::Type("type", "String", v))
        }
    }
}

/// Information on a slave nameserver.
#[derive(Clone, Debug)]
pub struct SlaveDns {
    pub hostname: String,
    pub address: String,
}

impl TryFrom<xmlrpc::Value> for SlaveDns {
    type Error = Error;
    fn try_from(v: xmlrpc::Value) -> Result<Self> {
        if let xmlrpc::Value::Struct(map) = v {
            let slave = Self {
                hostname: get_str(&map, "name")?,
                address: get_str(&map, "ip")?,
            };

            Ok(slave)
        } else {
            Err(Error::Type("slaveDns", "Struct", v))
        }
    }
}

/// Type of URL redirect. Can be a HTTP 301, 302 or frame.
#[derive(Clone, Debug)]
pub enum UrlRdrType {
    Permanent,
    Temporary,
    Frame,
}

impl fmt::Display for UrlRdrType {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let url_rdr_type = match self {
            Self::Permanent => "HEADER301",
            Self::Temporary => "HEADER302",
            Self::Frame => "FRAME",
        };

        fmt.write_str(url_rdr_type)
    }
}

impl TryFrom<String> for UrlRdrType {
    type Error = Error;
    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "HEADER301" => Ok(Self::Permanent),
            "HEADER302" => Ok(Self::Temporary),
            "FRAME" => Ok(Self::Frame),
            _ => Err(Error::BadVariant("UrlRdrType", s)),
        }
    }
}

/// A nameserver record. Contains DNS information as well as INWX metadata.
#[derive(Clone, Debug)]
pub struct Record {
    pub id: i32,
    pub name: String,
    pub record_type: RecordType,
    pub content: String,
    pub ttl: i32,
    pub priority: i32,
    pub url_rdr_type: UrlRdrType,
    pub url_rdr_title: String,
    pub url_rdr_desc: String,
    pub url_rdr_keywords: String,
    pub url_rdr_favicon: String,
    pub url_append: bool,
}

impl TryFrom<xmlrpc::Value> for Record {
    type Error = Error;
    fn try_from(v: xmlrpc::Value) -> Result<Self> {
        if let xmlrpc::Value::Struct(map) = v {
            let record = Self {
                id: get_i32(&map, "id")?,
                name: get_str(&map, "name")?,
                record_type: get_str(&map, "type")?.try_into()?,
                content: get_str(&map, "content")?,
                ttl: get_i32(&map, "ttl")?,
                priority: get_i32(&map, "prio")?,
                url_rdr_type: get_str(&map, "urlRedirectType")?.try_into()?,
                url_rdr_title: get_str(&map, "urlRedirectTitle")?,
                url_rdr_desc: get_str(&map, "urlRedirectDescription")?,
                url_rdr_keywords: get_str(&map, "urlRedirectKeywords")?,
                url_rdr_favicon: get_str(&map, "urlRedirectFavIcon")?,
                url_append: get_bool(&map, "urlAppend")?,
            };

            Ok(record)
        } else {
            Err(Error::Type("record", "Struct", v))
        }
    }
}

/// The records that match a search.
#[derive(Clone, Debug)]
pub struct RecordInfo {
    pub domain_id: i32,
    pub domain_name: String,
    pub domain_type: DomainType,
    pub master_address: String,
    pub last_zone_check: DateTime,
    pub slave_dns: SlaveDns,
    pub soa_serial: String,
    pub records: Vec<Record>,
}
