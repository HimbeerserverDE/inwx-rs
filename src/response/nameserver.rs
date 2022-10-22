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

impl From<UrlRdrType> for xmlrpc::Value {
    fn from(url_rdr_type: UrlRdrType) -> Self {
        url_rdr_type.to_string().into()
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
    pub url_rdr_type: Option<UrlRdrType>,
    pub url_rdr_title: Option<String>,
    pub url_rdr_desc: Option<String>,
    pub url_rdr_keywords: Option<String>,
    pub url_rdr_favicon: Option<String>,
    pub url_append: Option<bool>,
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
                url_rdr_type: match get_str(&map, "urlRedirectType").ok() {
                    Some(url_rdr_type) => url_rdr_type.try_into().ok(),
                    None => None,
                },
                url_rdr_title: get_str(&map, "urlRedirectTitle").ok(),
                url_rdr_desc: get_str(&map, "urlRedirectDescription").ok(),
                url_rdr_keywords: get_str(&map, "urlRedirectKeywords").ok(),
                url_rdr_favicon: get_str(&map, "urlRedirectFavIcon").ok(),
                url_append: get_bool(&map, "urlAppend").ok(),
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
    pub domain_id: Option<i32>,
    pub domain_name: Option<String>,
    pub domain_type: Option<DomainType>,
    pub master_address: Option<String>,
    pub last_zone_check: Option<DateTime>,
    pub slave_dns: Option<SlaveDns>,
    pub soa_serial: Option<String>,
    pub records: Option<Vec<Record>>,
}

impl TryFrom<Response> for RecordInfo {
    type Error = Error;
    fn try_from(resp: Response) -> Result<Self> {
        let info = Self {
            domain_id: get_i32(&resp.data, "roId").ok(),
            domain_name: get_str(&resp.data, "domain").ok(),
            domain_type: match get_str(&resp.data, "type").ok() {
                Some(domain_type) => domain_type.try_into().ok(),
                None => None,
            },
            master_address: get_str(&resp.data, "masterIp").ok(),
            last_zone_check: get_datetime(&resp.data, "lastZoneCheck").ok(),
            slave_dns: match get_map(&resp.data, "slaveDns").ok() {
                Some(slave_dns) => slave_dns.try_into().ok(),
                None => None,
            },
            soa_serial: get_str(&resp.data, "SOAserial").ok(),
            records: match get_array(&resp.data, "record").ok() {
                Some(records) => Some(
                    records
                        .iter()
                        .map(|v| v.to_owned().try_into())
                        .collect::<Result<Vec<Record>>>()?,
                ),
                None => None,
            },
        };

        Ok(info)
    }
}
