//! A WARC (Web ARChive) library

use chrono::UTC;
use hyper::header::{Headers, ContentLength};
use header::{WARCRecordID, WARCType, WARCRecordType, WARCDate};
use std::fmt;
use uuid::Uuid;

pub struct WARCRecord {
    pub version: String,
    pub headers: Headers,
}

impl WARCRecord {
    pub fn new() -> Self {
        let mut record = WARCRecord {
            version: "1.0".to_owned(),
            headers: Headers::new()
        };

        let id = format!("<urn:uuid:{}>", Uuid::new_v4());
        record.headers.set(WARCRecordID(id));

        record.headers.set(ContentLength(0));

        let record_type = WARCRecordType::Unknown("undefined".to_owned());
        record.headers.set(WARCType(record_type));

        record.headers.set(WARCDate(UTC::now()));

        record
    }
}

impl fmt::Display for WARCRecord {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = writeln!(f, "WARC/{}", self.version);

        for header in self.headers.iter() {
            let _ = writeln!(f, "{}", header);
        }

        Ok(())
    }
}
