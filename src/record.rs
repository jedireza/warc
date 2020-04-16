use crate::{WarcHeaders, WarcHeadersRef};
use chrono::Utc;
use std::fmt;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct WarcRecord {
    pub version: String,
    pub headers: WarcHeaders,
    pub body: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WarcRecordRef<'a> {
    pub version: &'a str,
    pub headers: WarcHeadersRef<'a>,
    pub body: &'a [u8],
}

impl WarcRecord {
    pub fn make_uuid() -> String {
        format!("<{}>", Uuid::new_v4().to_urn())
    }

    pub fn make_date() -> String {
        format!("{}", Utc::now())
    }
}

impl<'a> From<WarcRecordRef<'a>> for WarcRecord {
    fn from(record_ref: WarcRecordRef) -> Self {
        WarcRecord {
            version: record_ref.version.to_owned(),
            headers: WarcHeaders::from(record_ref.headers),
            body: record_ref.body.to_owned(),
        }
    }
}

impl fmt::Display for WarcRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "WARC/{}", self.version)?;

        write!(f, "{}", self.headers)?;

        if self.body.len() > 0 {
            writeln!(f, "\n{}", String::from_utf8_lossy(&self.body))?;
        }

        writeln!(f, "")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::header::WARC_TYPE;
    use crate::{WarcHeader, WarcHeaders, WarcRecord, WarcRecordType};

    #[test]
    fn create() {
        let record = WarcRecord {
            version: "1.0".to_owned(),
            headers: WarcHeaders::new(vec![]),
            body: vec![],
        };

        assert_eq!(record.body.len(), 0);
    }

    #[test]
    fn create_with_headers() {
        let record = WarcRecord {
            version: "1.0".to_owned(),
            headers: WarcHeaders::new(vec![WarcHeader::new(
                WARC_TYPE,
                WarcRecordType::WarcInfo.to_string(),
            )]),
            body: vec![],
        };

        assert_eq!(record.headers.len(), 1);
    }
}
