use chrono::prelude::*;

use warc::header::WarcHeader;
use warc::{RawHeader, Record, RecordType, WarcWriter};

fn main() -> Result<(), std::io::Error> {
    let date = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
    let body = format!("wrote to the file on {}", date);
    let body = body.into_bytes();

    let headers = RawHeader {
        version: "1.0".to_owned(),
        headers: vec![
            (
                WarcHeader::RecordID,
                Record::generate_record_id().into_bytes(),
            ),
            (
                WarcHeader::WarcType,
                RecordType::WarcInfo.to_string().into_bytes(),
            ),
            (WarcHeader::Date, date.into_bytes()),
            (WarcHeader::IPAddress, "127.0.0.1".to_owned().into_bytes()),
            (
                WarcHeader::ContentLength,
                body.len().to_string().into_bytes(),
            ),
        ]
        .into_iter()
        .collect(),
    };

    let mut file = WarcWriter::from_path("warc_example.warc")?;

    let bytes_written = file.write_raw(headers, &body)?;

    println!("{} bytes written.", bytes_written);

    Ok(())
}
