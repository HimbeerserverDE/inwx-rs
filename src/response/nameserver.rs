use super::Response;
use crate::common::nameserver::SlaveDns;

use serde_derive::{Deserialize, Serialize};

type DateTime = String;

/// A nameserver record. Contains DNS information as well as INWX metadata.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Record {
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub record_type: String,
    pub content: String,
    pub ttl: i32,
    #[serde(rename = "prio")]
    pub priority: i32,
    #[serde(rename = "urlRedirectType")]
    pub url_rdr_type: Option<String>,
    #[serde(rename = "urlRedirectTitle")]
    pub url_rdr_title: Option<String>,
    #[serde(rename = "urlRedirectDescription")]
    pub url_rdr_desc: Option<String>,
    #[serde(rename = "urlRedirectKeywords")]
    pub url_rdr_keywords: Option<String>,
    #[serde(rename = "urlRedirectFavIcon")]
    pub url_rdr_favicon: Option<String>,
    #[serde(rename = "urlAppend")]
    pub url_append: Option<bool>,
}

/// The records that match a search.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RecordInfo {
    #[serde(rename = "roId")]
    pub domain_id: Option<i32>,
    #[serde(rename = "domain")]
    pub domain_name: Option<String>,
    #[serde(rename = "type")]
    pub domain_type: Option<String>,
    #[serde(rename = "masterIp")]
    pub master_address: Option<String>,
    #[serde(rename = "lastZoneCheck")]
    pub last_zone_check: Option<DateTime>,
    #[serde(rename = "slaveDns")]
    pub slave_dns: Option<SlaveDns>,
    #[serde(rename = "SOAserial")]
    pub soa_serial: Option<String>,
    #[serde(rename = "record")]
    pub records: Option<Vec<Record>>,
}

impl Response for RecordInfo {
    fn unwrap(wrapped: Option<Self>) -> Self {
        wrapped.unwrap()
    }
}
