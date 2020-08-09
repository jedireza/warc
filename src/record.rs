use chrono::Utc;
use std::collections::HashMap;
use std::fmt;
use uuid::Uuid;

use crate::header::WarcHeader;
use crate::Error as WarcError;

/// A single WARC record parsed from a data stream.
#[derive(Clone, Debug, PartialEq)]
pub struct Record {
    /// The WARC standard version this record conforms to.
    pub version: String,
    /// A set of all headers that are part of this record.
    pub headers: HashMap<crate::header::WarcHeader, Vec<u8>>,
    /// The data body of this record.
    pub body: Vec<u8>,
}

impl Record {
    /// Return a new UUID as a string value for the WARC-Record-ID header.
    pub fn make_uuid() -> String {
        format!("<{}>", Uuid::new_v4().to_urn())
    }

    /// Return the current timestamp as a string value for the WARC-Date header.
    pub fn make_date() -> String {
        format!("{}", Utc::now())
    }

    pub fn verify(&self) -> Result<(), WarcError> {
        for header in vec![
            WarcHeader::WarcType,
            WarcHeader::RecordID,
            WarcHeader::ContentLength,
            WarcHeader::Date,
        ]
        .into_iter()
        {
            if !self.headers.contains_key(&header) {
                return Err(WarcError::MissingHeader(header));
            }
        }
        Ok(())
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "WARC/{}", self.version)?;

        for (token, value) in self.headers.iter() {
            writeln!(
                f,
                "{}: {}",
                token.to_string(),
                String::from_utf8_lossy(value)
            )?;
        }
        writeln!(f, "")?;

        if self.body.len() > 0 {
            writeln!(f, "\n{}", String::from_utf8_lossy(&self.body))?;
        }

        writeln!(f, "")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::header::WarcHeader;
    use crate::{Record, RecordType};
    use std::collections::HashMap;

    #[test]
    fn create() {
        let record = Record {
            version: "1.0".to_owned(),
            headers: HashMap::new(),
            body: vec![],
        };

        assert_eq!(record.body.len(), 0);
    }

    #[test]
    fn create_with_headers() {
        let record = Record {
            version: "1.0".to_owned(),
            headers: vec![(
                WarcHeader::WarcType,
                RecordType::WarcInfo.to_string().into_bytes(),
            )]
            .into_iter()
            .collect(),
            body: vec![],
        };

        assert_eq!(record.headers.len(), 1);
    }

    #[test]
    fn verify_ok() {
        let record = Record {
            version: "1.0".to_owned(),
            headers: vec![
                (WarcHeader::WarcType, b"dunno".to_vec()),
                (WarcHeader::ContentLength, b"5".to_vec()),
                (
                    WarcHeader::RecordID,
                    b"<urn:test:basic-record:record-0>".to_vec(),
                ),
                (WarcHeader::Date, b"2020-07-08T02:52:55Z".to_vec()),
            ]
            .into_iter()
            .collect(),
            body: b"12345".to_vec(),
        };

        assert!(record.verify().is_ok());
    }

    #[test]
    fn verify_missing_type() {
        let record = Record {
            version: "1.0".to_owned(),
            headers: vec![
                (WarcHeader::ContentLength, b"5".to_vec()),
                (
                    WarcHeader::RecordID,
                    b"<urn:test:basic-record:record-0>".to_vec(),
                ),
                (WarcHeader::Date, b"2020-07-08T02:52:55Z".to_vec()),
            ]
            .into_iter()
            .collect(),
            body: b"12345".to_vec(),
        };

        assert!(record.verify().is_err());
    }

    #[test]
    fn verify_missing_content_length() {
        let record = Record {
            version: "1.0".to_owned(),
            headers: vec![
                (WarcHeader::WarcType, b"dunno".to_vec()),
                (
                    WarcHeader::RecordID,
                    b"<urn:test:basic-record:record-0>".to_vec(),
                ),
                (WarcHeader::Date, b"2020-07-08T02:52:55Z".to_vec()),
            ]
            .into_iter()
            .collect(),
            body: b"12345".to_vec(),
        };

        assert!(record.verify().is_err());
    }

    #[test]
    fn verify_missing_record_id() {
        let record = Record {
            version: "1.0".to_owned(),
            headers: vec![
                (WarcHeader::WarcType, b"dunno".to_vec()),
                (WarcHeader::ContentLength, b"5".to_vec()),
                (WarcHeader::Date, b"2020-07-08T02:52:55Z".to_vec()),
            ]
            .into_iter()
            .collect(),
            body: b"12345".to_vec(),
        };

        assert!(record.verify().is_err());
    }

    #[test]
    fn verify_missing_date() {
        let record = Record {
            version: "1.0".to_owned(),
            headers: vec![
                (WarcHeader::WarcType, b"dunno".to_vec()),
                (WarcHeader::ContentLength, b"5".to_vec()),
                (
                    WarcHeader::RecordID,
                    b"<urn:test:basic-record:record-0>".to_vec(),
                ),
            ]
            .into_iter()
            .collect(),
            body: b"12345".to_vec(),
        };

        assert!(record.verify().is_err());
    }
}
