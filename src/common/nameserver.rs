use serde_derive::{Deserialize, Serialize};

/// Information on a slave nameserver.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SlaveDns {
    pub hostname: String,
    pub address: String,
}
