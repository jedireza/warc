use warc::header::{CONTENT_LENGTH, WARC_DATE, WARC_IP_ADDRESS, WARC_RECORD_ID, WARC_TYPE};
use warc::{WarcFile, WarcHeader, WarcHeaders, WarcRecord, WarcRecordType};

fn main() -> Result<(), std::io::Error> {
    let date = WarcRecord::make_date();
    let body = format!("wrote to the file on {}", date);
    let body = body.into_bytes();

    let record = WarcRecord {
        version: "1.0".to_owned(),
        headers: WarcHeaders::new(vec![
            WarcHeader::new(WARC_RECORD_ID, WarcRecord::make_uuid()),
            WarcHeader::new(WARC_TYPE, WarcRecordType::WarcInfo.to_string()),
            WarcHeader::new(WARC_DATE, date),
            WarcHeader::new(WARC_IP_ADDRESS, "127.0.0.1".to_owned()),
            WarcHeader::new(CONTENT_LENGTH, body.len().to_string()),
        ]),
        body: body,
    };

    let mut file = WarcFile::open("warc_example.warc")?;

    let bytes_written = file.write(&record)?;

    println!("{} bytes written.", bytes_written);

    Ok(())
}
