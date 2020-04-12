use warc::header::{WARC_DATE, WARC_IP_ADDRESS, WARC_RECORD_ID, WARC_TYPE};
use warc::{WarcFile, WarcHeader, WarcRecord, WarcRecordType};

fn main() -> Result<(), std::io::Error> {
    let date = WarcRecord::make_date();
    let body = format!("wrote to the file on {}", date);

    let record = WarcRecord {
        version: "1.0".to_owned(),
        headers: vec![
            WarcHeader::new(WARC_RECORD_ID, WarcRecord::make_uuid()),
            WarcHeader::new(WARC_TYPE, WarcRecordType::WarcInfo.to_string()),
            WarcHeader::new(WARC_DATE, date),
            WarcHeader::new(WARC_IP_ADDRESS, "127.0.0.1".to_owned()),
        ],
        body: body.into_bytes(),
    };

    let mut file = WarcFile::open("warc_example.warc")?;

    let bytes_written = file.write(record)?;

    println!("{} bytes written.", bytes_written);

    Ok(())
}
