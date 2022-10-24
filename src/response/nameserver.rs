use super::Response;
use crate::common::nameserver::{DomainType, RecordType, SlaveDns, UrlRdrType};
use crate::common::*;
use crate::{Error, Result};

use iso8601::DateTime;

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
                id: get_i32(&map, "id".into())?,
                name: get_str(&map, "name".into())?,
                record_type: get_str(&map, "type".into())?.try_into()?,
                content: get_str(&map, "content".into())?,
                ttl: get_i32(&map, "ttl".into())?,
                priority: get_i32(&map, "prio".into())?,
                url_rdr_type: match get_str(&map, "urlRedirectType".into()).ok() {
                    Some(url_rdr_type) => url_rdr_type.try_into().ok(),
                    None => None,
                },
                url_rdr_title: get_str(&map, "urlRedirectTitle".into()).ok(),
                url_rdr_desc: get_str(&map, "urlRedirectDescription".into()).ok(),
                url_rdr_keywords: get_str(&map, "urlRedirectKeywords".into()).ok(),
                url_rdr_favicon: get_str(&map, "urlRedirectFavIcon".into()).ok(),
                url_append: get_bool(&map, "urlAppend".into()).ok(),
            };

            Ok(record)
        } else {
            Err(Error::Type("record".into(), "Struct".into(), v))
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
            domain_id: get_i32(&resp.data, "roId".into()).ok(),
            domain_name: get_str(&resp.data, "domain".into()).ok(),
            domain_type: match get_str(&resp.data, "type".into()).ok() {
                Some(domain_type) => domain_type.try_into().ok(),
                None => None,
            },
            master_address: get_str(&resp.data, "masterIp".into()).ok(),
            last_zone_check: { get_datetime(&resp.data, "lastZoneCheck".into()).ok() },
            slave_dns: match get_map(&resp.data, "slaveDns".into()).ok() {
                Some(slave_dns) => slave_dns.try_into().ok(),
                None => None,
            },
            soa_serial: get_str(&resp.data, "SOAserial".into()).ok(),
            records: match get_array(&resp.data, "record".into()).ok() {
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
