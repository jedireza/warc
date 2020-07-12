use chrono::Utc;
use std::collections::HashMap;
use std::fmt;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct Record {
    pub version: String,
    pub headers: HashMap<crate::header::WarcHeader, Vec<u8>>,
    pub body: Vec<u8>,
}

impl Record {
    pub fn make_uuid() -> String {
        format!("<{}>", Uuid::new_v4().to_urn())
    }

    pub fn make_date() -> String {
        format!("{}", Utc::now())
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "WARC/{}", self.version)?;

        for (token, value) in self.headers.iter() {
            writeln!(f, "{}: {}", token.to_string(), String::from_utf8_lossy(value))?;
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
}
