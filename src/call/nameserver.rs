use crate::response::nameserver::RecordInfo as RecordInfoResponse;
use super::*;

use serde_derive::{Deserialize, Serialize};

/// Optional search constraints to find nameserver records
/// the account has access to.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RecordInfo {
    #[serde(rename = "domain")]
    pub domain_name: Option<String>,
    #[serde(rename = "roId")]
    pub domain_id: Option<i32>,
    #[serde(rename = "recordId")]
    pub record_id: Option<i32>,
    #[serde(rename = "type")]
    pub record_type: Option<String>,
    pub name: Option<String>,
    pub content: Option<String>,
    pub ttl: Option<i32>,
    #[serde(rename = "prio")]
    pub priority: Option<i32>,
}

impl Call for RecordInfo {
    fn method_name(&self) -> String {
        String::from("nameserver.info")
    }

    fn expected(&self) -> Vec<i32> {
        vec![1000]
    }
}

impl Response<RecordInfoResponse> for RecordInfo {}

/// Update the records with the specified IDs.
/// Any `None` variants will remain unchanged.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RecordUpdate {
    #[serde(rename = "id")]
    pub ids: Vec<i32>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub record_type: Option<String>,
    pub content: Option<String>,
    pub ttl: Option<i32>,
    #[serde(rename = "prio")]
    pub priority: Option<i32>,
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
    #[serde(rename = "testing")]
    pub testing_mode: bool,
}

impl Call for RecordUpdate {
    fn method_name(&self) -> String {
        String::from("nameserver.updateRecord")
    }

    fn expected(&self) -> Vec<i32> {
        vec![1000]
    }
}

impl Response<()> for RecordUpdate {}
