use super::*;
use crate::{Error, Result};

use std::fmt;

/// The DNS record type.
#[derive(Clone, Debug)]
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
            Self::A => write!(fmt, "A"),
            Self::Aaaa => write!(fmt, "AAAA"),
            Self::Afsdb => write!(fmt, "AFSDB"),
            Self::Alias => write!(fmt, "ALIAS"),
            Self::Caa => write!(fmt, "CAA"),
            Self::Cert => write!(fmt, "CERT"),
            Self::Cname => write!(fmt, "CNAME"),
            Self::Hinfo => write!(fmt, "HINFO"),
            Self::Key => write!(fmt, "KEY"),
            Self::Loc => write!(fmt, "LOC"),
            Self::Mx => write!(fmt, "MX"),
            Self::NaPtr => write!(fmt, "NAPTR"),
            Self::Ns => write!(fmt, "NS"),
            Self::OpenPgpKey => write!(fmt, "OPENPGPKEY"),
            Self::Ptr => write!(fmt, "PTR"),
            Self::Rp => write!(fmt, "RP"),
            Self::SmimeA => write!(fmt, "SMIMEA"),
            Self::Soa => write!(fmt, "SOA"),
            Self::Srv => write!(fmt, "SRV"),
            Self::Sshfp => write!(fmt, "SSHFP"),
            Self::Tlsa => write!(fmt, "TLSA"),
            Self::Txt => write!(fmt, "TXT"),
            Self::Uri => write!(fmt, "URI"),
            Self::Url => write!(fmt, "URL"),
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
            "SRV" => Ok(Self::Srv),
            "SSHFP" => Ok(Self::Sshfp),
            "TLSA" => Ok(Self::Tlsa),
            "TXT" => Ok(Self::Txt),
            "URI" => Ok(Self::Uri),
            "URL" => Ok(Self::Url),
            _ => Err(Error::BadVariant("RecordType".into(), s)),
        }
    }
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
            _ => Err(Error::BadVariant("DomainType".into(), s)),
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
        Ok(Self {
            hostname: get_str(&map, "name".into())?,
            address: get_str(&map, "ip".into())?,
        })
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
        fmt.write_str(match self {
            Self::Permanent => "HEADER301",
            Self::Temporary => "HEADER302",
            Self::Frame => "FRAME",
        })
    }
}

impl TryFrom<String> for UrlRdrType {
    type Error = Error;
    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "HEADER301" => Ok(Self::Permanent),
            "HEADER302" => Ok(Self::Temporary),
            "FRAME" => Ok(Self::Frame),
            _ => Err(Error::BadVariant("UrlRdrType".into(), s)),
        }
    }
}

impl From<UrlRdrType> for xmlrpc::Value {
    fn from(url_rdr_type: UrlRdrType) -> Self {
        url_rdr_type.to_string().into()
    }
}
