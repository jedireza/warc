//! A WARC (Web ARChive) library

use ::header::WarcDate;
use ::header::WarcRecordID;
use ::header::WarcRecordType;
use ::header::WarcType;
use ::WarcVersion;
use chrono::Utc;
use hyper::header::ContentLength;
use hyper::header::Headers;
use std::fmt;
use uuid::Uuid;

pub struct WarcRecord {
    pub version: WarcVersion,
    pub headers: Headers,
    pub body: Vec<u8>,
}

impl WarcRecord {
    pub fn new() -> Self {
        let mut record = WarcRecord {
            version: WarcVersion::Warc10,
            headers: Headers::new(),
            body: Vec::new(),
        };
        let id = format!("<{}>", Uuid::new_v4().urn());
        let record_type = WarcRecordType::Unknown("undefined".to_owned());

        record.headers.set(WarcRecordID(id));
        record.headers.set(ContentLength(0));
        record.headers.set(WarcType(record_type));
        record.headers.set(WarcDate(Utc::now()));

        record
    }

    pub fn set_body(&mut self, body: Vec<u8>) {
        self.headers.set(ContentLength(body.len() as u64));
        self.body = body;
    }
}

impl fmt::Display for WarcRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.version)?;

        for header in self.headers.iter() {
            write!(f, "{}", header)?;
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
    use ::WarcRecord;
    use ::header::ContentLength;
    use ::header::WarcRecordType;
    use ::header::WarcType;

    #[test]
    fn create_new() {
        let rec = WarcRecord::new();

        assert_eq!(rec.body.len(), 0);
        assert_eq!(rec.headers.get::<ContentLength>().unwrap().0, 0);
        assert_eq!(
            rec.headers.get::<WarcType>().unwrap().0,
            WarcRecordType::Unknown("undefined".to_owned())
        );
    }

    #[test]
    fn set_headers() {
        let mut rec = WarcRecord::new();
        rec.headers.set(WarcType(WarcRecordType::WarcInfo));

        assert_eq!(
            rec.headers.get::<WarcType>().unwrap().0,
            WarcRecordType::WarcInfo
        );
    }

    #[test]
    fn set_body() {
        let mut rec = WarcRecord::new();
        rec.set_body("hello world! 👋".to_owned().into_bytes());

        assert_eq!(rec.body, "hello world! 👋".to_owned().into_bytes());
        assert_eq!(rec.headers.get::<ContentLength>().unwrap().0, 17);
    }
}
