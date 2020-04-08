use crate::WarcHeaders;
use chrono::Utc;
use std::fmt;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct WarcRecord<'a> {
    pub version: &'a str,
    pub headers: WarcHeaders<'a>,
    pub body: &'a [u8],
}

impl<'a> WarcRecord<'a> {
    pub fn uuid() -> String {
        format!("<{}>", Uuid::new_v4().to_urn())
    }

    pub fn now() -> String {
        format!("{}", Utc::now())
    }
}

impl<'a> fmt::Display for WarcRecord<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "WARC/{}", &self.version)?;

        for (key, value) in self.headers.iter() {
            println!("{}: {}", key, value);
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
    use crate::{WarcRecord, WarcRecordType};

    #[test]
    fn create() {
        let record = WarcRecord {
            version: "1.0",
            headers: vec![],
            body: &[],
        };

        assert_eq!(record.body.len(), 0);
    }

    #[test]
    fn create_with_headers() {
        let warc_type = WarcRecordType::WarcInfo.to_string();
        let record = WarcRecord {
            version: "1.0",
            headers: vec![(WARC_TYPE, warc_type.as_str())],
            body: &[],
        };

        assert_eq!(record.headers.len(), 1);
    }
}
