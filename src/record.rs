use crate::WarcHeaders;
use chrono::Utc;
use std::fmt;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct WarcRecord<'a> {
    pub version: &'a [u8],
    pub headers: WarcHeaders<'a>,
    pub body: &'a [u8],
}

impl<'a> WarcRecord<'a> {
    pub fn make_uuid() -> String {
        format!("<{}>", Uuid::new_v4().to_urn())
    }

    pub fn make_date() -> String {
        format!("{}", Utc::now())
    }
}

impl<'a> fmt::Display for WarcRecord<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "WARC/{}", String::from_utf8_lossy(self.version))?;

        for header in self.headers.iter() {
            writeln!(
                f,
                "{}: {}",
                header.key,
                String::from_utf8_lossy(header.value)
            )?;
        }

        if self.body.len() > 0 {
            writeln!(f, "\n{}", String::from_utf8_lossy(self.body))?;
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
            version: "1.0".as_bytes(),
            headers: vec![],
            body: &[],
        };

        assert_eq!(record.body.len(), 0);
    }

    #[test]
    fn create_with_headers() {
        let warc_type = WarcRecordType::WarcInfo.to_string();
        let record = WarcRecord {
            version: "1.0".as_bytes(),
            headers: vec![WarcHeader::new(WARC_TYPE, warc_type.as_bytes())],
            body: &[],
        };

        assert_eq!(record.headers.len(), 1);
    }
}
