use super::Call;
use crate::common::nameserver::{RecordType, UrlRdrType};

use std::collections::BTreeMap;

/// Optional search constraints to find nameserver records
/// the account has access to.
#[derive(Clone, Debug)]
pub struct RecordInfo {
    pub domain_name: Option<String>,
    pub domain_id: Option<i32>,
    pub record_id: Option<i32>,
    pub record_type: Option<RecordType>,
    pub name: Option<String>,
    pub content: Option<String>,
    pub ttl: Option<i32>,
    pub priority: Option<i32>,
}

impl From<RecordInfo> for xmlrpc::Value {
    fn from(info: RecordInfo) -> Self {
        let mut map = BTreeMap::new();

        if let Some(domain_name) = info.domain_name {
            map.insert("domain".into(), domain_name.into());
        }

        if let Some(domain_id) = info.domain_id {
            map.insert("roId".into(), domain_id.into());
        }

        if let Some(record_id) = info.record_id {
            map.insert("recordId".into(), record_id.into());
        }

        if let Some(record_type) = info.record_type {
            map.insert("type".into(), record_type.into());
        }

        if let Some(content) = info.content {
            map.insert("content".into(), content.into());
        }

        if let Some(ttl) = info.ttl {
            map.insert("ttl".into(), ttl.into());
        }

        if let Some(priority) = info.priority {
            map.insert("prio".into(), priority.into());
        }

        xmlrpc::Value::Struct(map)
    }
}

impl Call for RecordInfo {
    fn method_name(&self) -> &'static str {
        "nameserver.info"
    }

    fn expected(&self) -> &'static [i32] {
        &[1000]
    }
}

/// Update the records with the specified IDs.
/// Any `None` variants will remain unchanged.
#[derive(Clone, Debug)]
pub struct RecordUpdate {
    pub ids: Vec<i32>,
    pub name: Option<String>,
    pub record_type: Option<RecordType>,
    pub content: Option<String>,
    pub ttl: Option<i32>,
    pub priority: Option<i32>,
    pub url_rdr_type: Option<UrlRdrType>,
    pub url_rdr_title: Option<String>,
    pub url_rdr_desc: Option<String>,
    pub url_rdr_keywords: Option<String>,
    pub url_rdr_favicon: Option<String>,
    pub url_append: Option<bool>,
    pub testing_mode: bool,
}

impl From<RecordUpdate> for xmlrpc::Value {
    fn from(update: RecordUpdate) -> Self {
        let mut map = BTreeMap::new();

        map.insert(
            "id".into(),
            xmlrpc::Value::Array(update.ids.iter().map(|v| xmlrpc::Value::from(*v)).collect()),
        );

        if let Some(name) = update.name {
            map.insert("name".into(), name.into());
        }

        if let Some(record_type) = update.record_type {
            map.insert("type".into(), record_type.into());
        }

        if let Some(content) = update.content {
            map.insert("content".into(), content.into());
        }

        if let Some(ttl) = update.ttl {
            map.insert("ttl".into(), ttl.into());
        }

        if let Some(priority) = update.priority {
            map.insert("prio".into(), priority.into());
        }

        if let Some(url_rdr_type) = update.url_rdr_type {
            map.insert("urlRedirectType".into(), url_rdr_type.into());
        }

        if let Some(url_rdr_title) = update.url_rdr_title {
            map.insert("urlRedirectTitle".into(), url_rdr_title.into());
        }

        if let Some(url_rdr_desc) = update.url_rdr_desc {
            map.insert("urlRedirectDescription".into(), url_rdr_desc.into());
        }

        if let Some(url_rdr_keywords) = update.url_rdr_keywords {
            map.insert("urlRedirectKeywords".into(), url_rdr_keywords.into());
        }

        if let Some(url_rdr_favicon) = update.url_rdr_favicon {
            map.insert("urlRedirectFavIcon".into(), url_rdr_favicon.into());
        }

        if let Some(url_append) = update.url_append {
            map.insert("urlAppend".into(), url_append.into());
        }

        map.insert("testing".into(), update.testing_mode.into());

        xmlrpc::Value::Struct(map)
    }
}

impl Call for RecordUpdate {
    fn method_name(&self) -> &'static str {
        "nameserver.updateRecord"
    }

    fn expected(&self) -> &'static [i32] {
        &[1000]
    }
}
