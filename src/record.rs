use std::fmt;
use uuid::Uuid;

pub struct WarcRecord<'a> {
    pub version: &'a str,
    pub headers: Vec<(&'a str, &'a str)>,
    pub body: &'a [u8],
}

impl<'a> WarcRecord<'a> {
    pub fn uuid() -> String {
        format!("<{}>", Uuid::new_v4().to_urn())
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
    use crate::header::{CONTENT_LENGTH, WARC_TYPE};
    use crate::{WarcRecord, WarcRecordType};

    #[test]
    fn create_new() {
        let record = WarcRecord {
            version: "1.0",
            headers: vec![],
            body: &[],
        };

        assert_eq!(record.body.len(), 0);
    }

    #[test]
    fn set_headers() {
        let mut record = WarcRecord {
            version: "1.0",
            headers: vec![],
            body: &[],
        };

        let warc_type = WarcRecordType::WarcInfo.to_string();
        record.headers.push((WARC_TYPE, warc_type.as_str()));

        assert_eq!(record.headers.len(), 1);
    }

    #[test]
    fn set_body() {
        let mut record = WarcRecord {
            version: "1.0",
            headers: vec![],
            body: "hello world! 👋".as_bytes(),
        };

        assert_eq!(record.body, "hello world! 👋".as_bytes());
    }
}
