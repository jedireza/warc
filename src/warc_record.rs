use std::fmt;

pub struct WarcRecord {
    pub version: Vec<u8>,
    // pub headers: HeadersMap,
    pub body: Vec<u8>,
}

impl WarcRecord {
    pub fn new() -> Self {
        let record = WarcRecord {
            version: Vec::new(),
            // headers: HeaderMap::new(),
            body: Vec::new(),
        };

        record
    }

    pub fn set_body(&mut self, body: Vec<u8>) {
        // self.headers.set(ContentLength(body.len() as u64));
        self.body = body;
    }
}

impl fmt::Display for WarcRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\n{}", String::from_utf8_lossy(&self.version))?;

        // for header in self.headers.iter() {
        //     write!(f, "{}", header)?;
        // }

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

    #[test]
    fn create_new() {
        let rec = WarcRecord::new();

        assert_eq!(rec.body.len(), 0);
        // assert_eq!(rec.headers.get::<ContentLength>().unwrap().0, 0);
        // assert_eq!(
        //     rec.headers.get::<WarcType>().unwrap().0,
        //     WarcRecordType::Unknown("undefined".to_owned())
        // );
    }

    // #[test]
    // fn set_headers() {
    //     let mut rec = WarcRecord::new();
    //     rec.headers.set(WarcType(WarcRecordType::WarcInfo));
    //
    //     assert_eq!(
    //         rec.headers.get::<WarcType>().unwrap().0,
    //         WarcRecordType::WarcInfo
    //     );
    // }

    // #[test]
    // fn set_body() {
    //     let mut rec = WarcRecord::new();
    //     rec.set_body("hello world! 👋".to_owned().into_bytes());
    //
    //     assert_eq!(rec.body, "hello world! 👋".to_owned().into_bytes());
    //     assert_eq!(rec.headers.get::<ContentLength>().unwrap().0, 17);
    // }
}
