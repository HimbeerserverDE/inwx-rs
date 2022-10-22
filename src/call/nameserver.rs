use crate::{Error, Result};
use super::Call;

use std::collections::BTreeMap;
use std::fmt;

/// The DNS record type.
#[derive(Clone, Copy, Debug)]
pub enum RecordType {
    A,
    Aaaa,
    Afsdb,
    Alias,
    Caa,
    Cert,
    Cname,
    Hinfo,
    Key,
    Loc,
    Mx,
    NaPtr,
    Ns,
    OpenPgpKey,
    Ptr,
    Rp,
    SmimeA,
    Soa,
    Srv,
    Sshfp,
    Tlsa,
    Txt,
    Uri,
    Url,
}

impl fmt::Display for RecordType {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecordType::A => write!(fmt, "A"),
            RecordType::Aaaa => write!(fmt, "AAAA"),
            RecordType::Afsdb => write!(fmt, "AFSDB"),
            RecordType::Alias => write!(fmt, "ALIAS"),
            RecordType::Caa => write!(fmt, "CAA"),
            RecordType::Cert => write!(fmt, "CERT"),
            RecordType::Cname => write!(fmt, "CNAME"),
            RecordType::Hinfo => write!(fmt, "HINFO"),
            RecordType::Key => write!(fmt, "KEY"),
            RecordType::Loc => write!(fmt, "LOC"),
            RecordType::Mx => write!(fmt, "MX"),
            RecordType::NaPtr => write!(fmt, "NAPTR"),
            RecordType::Ns => write!(fmt, "NS"),
            RecordType::OpenPgpKey => write!(fmt, "OPENPGPKEY"),
            RecordType::Ptr => write!(fmt, "PTR"),
            RecordType::Rp => write!(fmt, "RP"),
            RecordType::SmimeA => write!(fmt, "SMIMEA"),
            RecordType::Soa => write!(fmt, "SOA"),
            RecordType::Srv => write!(fmt, "SRV"),
            RecordType::Sshfp => write!(fmt, "SSHFP"),
            RecordType::Tlsa => write!(fmt, "TLSA"),
            RecordType::Txt => write!(fmt, "TXT"),
            RecordType::Uri => write!(fmt, "URI"),
            RecordType::Url => write!(fmt, "URL"),
        }
    }
}

impl From<RecordType> for xmlrpc::Value {
    fn from(rt: RecordType) -> Self {
        xmlrpc::Value::String(rt.to_string())
    }
}

impl TryFrom<String> for RecordType {
    type Error = Error;
    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "A" => Ok(Self::A),
            "AAAA" => Ok(Self::Aaaa),
            "AFSDB" => Ok(Self::Afsdb),
            "ALIAS" => Ok(Self::Alias),
            "CAA" => Ok(Self::Caa),
            "CERT" => Ok(Self::Cert),
            "CNAME" => Ok(Self::Cname),
            "HINFO" => Ok(Self::Hinfo),
            "KEY" => Ok(Self::Key),
            "LOC" => Ok(Self::Loc),
            "MX" => Ok(Self::Mx),
            "NAPTR" => Ok(Self::NaPtr),
            "NS" => Ok(Self::Ns),
            "OPENPGPKEY" => Ok(Self::OpenPgpKey),
            "PTR" => Ok(Self::Ptr),
            "RP" => Ok(Self::Rp),
            "SMIMEA" => Ok(Self::SmimeA),
            "SOA" => Ok(Self::Soa),
            "SSHFP" => Ok(Self::Sshfp),
            "TLSA" => Ok(Self::Tlsa),
            "TXT" => Ok(Self::Txt),
            "URI" => Ok(Self::Uri),
            "URL" => Ok(Self::Url),
            _ => Err(Error::BadVariant("RecordType", s)),
        }
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
