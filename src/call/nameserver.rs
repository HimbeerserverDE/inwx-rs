use super::Call;

use std::collections::BTreeMap;
use std::fmt;

/// The DNS record type.
#[derive(Clone, Copy, Debug)]
pub enum RecordType {
    A,
    AAAA,
    AFSDB,
    ALIAS,
    CAA,
    CERT,
    CNAME,
    HINFO,
    KEY,
    LOC,
    MX,
    NAPTR,
    NS,
    OPENPGPKEY,
    PTR,
    RP,
    SMIMEA,
    SOA,
    SRV,
    SSHFP,
    TLSA,
    TXT,
    URI,
    URL,
}

impl fmt::Display for RecordType {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            A => write!(fmt, "A"),
            AAAA => write!(fmt, "AAAA"),
            AFSDB => write!(fmt, "AFSDB"),
            ALIAS => write!(fmt, "ALIAS"),
            CAA => write!(fmt, "CAA"),
            CERT => write!(fmt, "CERT"),
            CNAME => write!(fmt, "CNAME"),
            HINFO => write!(fmt, "HINFO"),
            KEY => write!(fmt, "KEY"),
            LOC => write!(fmt, "LOC"),
            MX => write!(fmt, "MX"),
            NAPTR => write!(fmt, "NAPTR"),
            NS => write!(fmt, "NS"),
            OPENPGPKEY => write!(fmt, "OPENPGPKEY"),
            PTR => write!(fmt, "PTR"),
            RP => write!(fmt, "RP"),
            SMIMEA => write!(fmt, "SMIMEA"),
            SOA => write!(fmt, "SOA"),
            SRV => write!(fmt, "SRV"),
            SSHFP => write!(fmt, "SSHFP"),
            TLSA => write!(fmt, "TLSA"),
            TXT => write!(fmt, "TXT"),
            URI => write!(fmt, "URI"),
            URL => write!(fmt, "URL"),
        }
    }
}

impl From<RecordType> for xmlrpc::Value {
    fn from(rt: RecordType) -> Self {
        xmlrpc::Value::String(rt.to_string())
    }
}

/// Search parameters to find nameserver records
/// the account has access to.
#[derive(Clone, Copy, Debug)]
pub struct RecordInfo<'a> {
    pub domain_name: &'a str,
    pub domain_id: i32,
    pub record_id: i32,
    pub record_type: RecordType,
    pub name: &'a str,
    pub content: &'a str,
    pub ttl: i32,
    pub priority: i32,
}

impl From<RecordInfo<'_>> for xmlrpc::Value {
    fn from(info: RecordInfo<'_>) -> Self {
        let mut map = BTreeMap::new();

        map.insert("domain".into(), info.domain_name.into());
        map.insert("roId".into(), info.domain_id.into());
        map.insert("recordId".into(), info.record_id.into());
        map.insert("type".into(), info.record_type.into());
        map.insert("content".into(), info.content.into());
        map.insert("ttl".into(), info.ttl.into());
        map.insert("prio".into(), info.priority.into());

        xmlrpc::Value::Struct(map)
    }
}

impl Call for RecordInfo<'_> {
    fn method_name(&self) -> &'static str {
        "nameserver.info"
    }

    fn expected(&self) -> &'static [i32] {
        &[1000]
    }
}
