use crate::header::{CONTENT_LENGTH, WARC_DATE, WARC_RECORD_ID, WARC_TYPE};
use chrono::Utc;
use http::header::HeaderMap;
use std::fmt;
use uuid::Uuid;

pub struct WarcRecord {
    pub version: Vec<u8>,
    pub headers: HeaderMap,
    pub body: Vec<u8>,
}

impl WarcRecord {
    pub fn new() -> Self {
        let mut record = WarcRecord {
            version: "WARC/1.0".to_owned().into_bytes(),
            headers: HeaderMap::new(),
            body: Vec::new(),
        };

        let id = format!("<{}>", Uuid::new_v4().to_urn());

        record.headers.insert(WARC_RECORD_ID, id.parse().unwrap());
        record.headers.insert(CONTENT_LENGTH, "0".parse().unwrap());
        record
            .headers
            .insert(WARC_TYPE, "undefined".parse().unwrap());
        record
            .headers
            .insert(WARC_DATE, Utc::now().to_string().parse().unwrap());

        record
    }

    pub fn set_body(&mut self, body: Vec<u8>) {
        self.headers
            .insert(CONTENT_LENGTH, body.len().to_string().parse().unwrap());
        self.body = body;
    }
}

impl fmt::Display for WarcRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\n{}", String::from_utf8_lossy(&self.version))?;

        for (key, value) in self.headers.iter() {
            println!("{}: {}", key.to_string(), value.to_str().unwrap());
        }

        if self.body.len() > 0 {
            writeln!(f, "\n{}", String::from_utf8_lossy(&self.body))?;
        }

        writeln!(f, "")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::header::{CONTENT_LENGTH, WARC_TYPE};
    use crate::WarcRecord;
    use crate::WarcRecordType;

    #[test]
    fn create_new() {
        let record = WarcRecord::new();

        assert_eq!(record.body.len(), 0);
        assert_eq!(record.headers[CONTENT_LENGTH], "0");
        assert_eq!(record.headers[WARC_TYPE], "undefined");
    }

    #[test]
    fn set_headers() {
        let mut record = WarcRecord::new();
        record.headers.insert(
            WARC_TYPE,
            WarcRecordType::WarcInfo.to_string().parse().unwrap(),
        );

        assert_eq!(
            record.headers[WARC_TYPE],
            WarcRecordType::WarcInfo.to_string()
        );
    }

    #[test]
    fn set_body() {
        let mut record = WarcRecord::new();
        record.set_body("hello world! 👋".to_owned().into_bytes());

        assert_eq!(record.body, "hello world! 👋".to_owned().into_bytes());
        assert_eq!(record.headers[CONTENT_LENGTH], "17");
    }
}
