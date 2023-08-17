use inwx::call::nameserver::{RecordInfo as RecordInfoCall, RecordUpdate};
use inwx::response::nameserver::RecordInfo as RecordInfoResponse;
use inwx::{Client, Endpoint};

const USER: &str = "inwxclient";
const PASS: &str = "inwx1@client";

#[test]
fn test_client() -> inwx::Result<()> {
    let clt = Client::login(Endpoint::Sandbox, String::from(USER), String::from(PASS))?;

    let records: RecordInfoResponse = clt.call(RecordInfoCall {
        domain_name: None,
        domain_id: None,
        record_id: Some(75503),
        record_type: None,
        name: None,
        content: None,
        ttl: None,
        priority: None,
    })?;

    println!("{:?}", records);

    clt.call(RecordUpdate {
        ids: vec![75503],
        name: None,
        record_type: None,
        content: Some(String::from("::1")),
        ttl: None,
        priority: None,
        url_rdr_type: None,
        url_rdr_title: None,
        url_rdr_desc: None,
        url_rdr_keywords: None,
        url_rdr_favicon: None,
        url_append: None,
        testing_mode: false,
    })?;

    Ok(())
}
