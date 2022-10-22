use super::Response;
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

fn get_datetime(map: &BTreeMap<String, xmlrpc::Value>, key: &'static str) -> Result<DateTime> {
    let value = map
        .get(key)
        .ok_or(Error::Inexistent(key))?
        .as_datetime()
        .ok_or_else(|| Error::Type(key, "DateTime", map.get(key).unwrap().clone()))?;

    Ok(value)
}

fn get_array(
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

fn get_map(
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

impl TryFrom<String> for DomainType {
    type Error = Error;
    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "MASTER" => Ok(Self::Master),
            "SLAVE" => Ok(Self::Slave),
            _ => Err(Error::BadVariant("DomainType", s)),
        }
    }
}

/// Information on a slave nameserver.
#[derive(Clone, Debug)]
pub struct SlaveDns {
    pub hostname: String,
    pub address: String,
}

impl TryFrom<BTreeMap<String, xmlrpc::Value>> for SlaveDns {
    type Error = Error;
    fn try_from(map: BTreeMap<String, xmlrpc::Value>) -> Result<Self> {
        let slave = Self {
            hostname: get_str(&map, "name")?,
            address: get_str(&map, "ip")?,
        };

        Ok(slave)
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

impl TryFrom<Response> for RecordInfo {
    type Error = Error;
    fn try_from(resp: Response) -> Result<Self> {
        let info = Self {
            domain_id: get_i32(&resp.data, "roId")?,
            domain_name: get_str(&resp.data, "domain")?,
            domain_type: get_str(&resp.data, "type")?.try_into()?,
            master_address: get_str(&resp.data, "masterIp")?,
            last_zone_check: get_datetime(&resp.data, "lastZoneCheck")?,
            slave_dns: get_map(&resp.data, "slaveDns")?.try_into()?,
            soa_serial: get_str(&resp.data, "SOAserial")?,
            records: get_array(&resp.data, "record")?
                .iter()
                .filter_map(|v| v.to_owned().try_into().ok())
                .collect(),
        };

        Ok(info)
    }
}
