use chrono::prelude::*;

use warc::header::WarcHeader;
use warc::{RawHeader, RawRecord, Record, RecordType};

fn main() {
    let body = "hello warc! 👋".to_owned().into_bytes();

    let record = RawRecord {
        headers: RawHeader {
            version: "1.0".to_owned(),
            headers: vec![
                (
                    WarcHeader::RecordID,
                    Record::generate_record_id().into_bytes(),
                ),
                (
                    WarcHeader::WarcType,
                    RecordType::WarcInfo.to_string().into_bytes(),
                ),
                (
                    WarcHeader::Date,
                    Utc::now()
                        .to_rfc3339_opts(SecondsFormat::Secs, true)
                        .into_bytes(),
                ),
                (WarcHeader::IPAddress, "127.0.0.1".to_owned().into_bytes()),
                (
                    WarcHeader::ContentLength,
                    body.len().to_string().into_bytes(),
                ),
            ]
            .into_iter()
            .collect(),
        },
        body: body,
    };

    print!("{}", record);
}
