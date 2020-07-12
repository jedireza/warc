use warc::header::WarcHeader;
use warc::{Record, RecordType};

fn main() {
    let body = "hello warc! 👋".to_owned().into_bytes();

    let record = Record {
        version: "1.0".to_owned(),
        headers: vec![
            (
                WarcHeader::RecordID,
                Record::make_uuid().to_owned().into_bytes(),
            ),
            (
                WarcHeader::WarcType,
                RecordType::WarcInfo.to_string().into_bytes(),
            ),
            (
                WarcHeader::Date,
                Record::make_date().into_bytes(),
            ),
            (
                WarcHeader::IPAddress,
                "127.0.0.1".to_owned().into_bytes(),
            ),
            (
                WarcHeader::ContentLength,
                body.len().to_string().into_bytes(),
            ),
        ]
        .into_iter()
        .collect(),
        body: body,
    };

    print!("{}", record);
}
