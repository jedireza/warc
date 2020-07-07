use warc::header::WarcHeader;
use warc::{Record, RecordType, WarcWriter};

fn main() -> Result<(), std::io::Error> {
    let date = Record::make_date();
    let body = format!("wrote to the file on {}", date);
    let body = body.into_bytes();

    let record = Record {
        version: "1.0".to_owned(),
        headers: vec![
            (WarcHeader::WARC_RECORD_ID, Record::make_uuid().into_bytes()),
            (
                WarcHeader::WARC_TYPE,
                RecordType::WarcInfo.to_string().into_bytes(),
            ),
            (WarcHeader::WARC_DATE, date.into_bytes()),
            (
                WarcHeader::WARC_IP_ADDRESS,
                "127.0.0.1".to_owned().into_bytes(),
            ),
            (
                WarcHeader::CONTENT_LENGTH,
                body.len().to_string().into_bytes(),
            ),
        ]
        .into_iter()
        .collect(),
        body: body,
    };

    let mut file = WarcWriter::from_path("warc_example.warc")?;

    let bytes_written = file.write(&record)?;

    println!("{} bytes written.", bytes_written);

    Ok(())
}
