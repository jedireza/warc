use crate::WarcHeaders;
use chrono::Utc;
use std::fmt;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct WarcRecord {
    pub version: String,
    pub headers: WarcHeaders,
    pub body: Vec<u8>,
}

impl WarcRecord {
    pub fn make_uuid() -> String {
        format!("<{}>", Uuid::new_v4().to_urn())
    }

    pub fn make_date() -> String {
        format!("{}", Utc::now())
    }
}

impl fmt::Display for WarcRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "WARC/{}", self.version)?;

        for header in self.headers.iter() {
            writeln!(
                f,
                "{}: {}",
                header.token,
                String::from_utf8_lossy(&header.value)
            )?;
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
    use crate::header::WARC_TYPE;
    use crate::{WarcHeader, WarcRecord, WarcRecordType};

    #[test]
    fn create() {
        let record = WarcRecord {
            version: "1.0".to_owned(),
            headers: vec![],
            body: vec![],
        };

        assert_eq!(record.body.len(), 0);
    }

    #[test]
    fn create_with_headers() {
        let record = WarcRecord {
            version: "1.0".to_owned(),
            headers: vec![WarcHeader::new(
                WARC_TYPE,
                WarcRecordType::WarcInfo.to_string(),
            )],
            body: vec![],
        };

        assert_eq!(record.headers.len(), 1);
    }
}
