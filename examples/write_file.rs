use warc::header::{CONTENT_LENGTH, WARC_DATE, WARC_IP_ADDRESS, WARC_RECORD_ID, WARC_TYPE};
use warc::{Record, RecordType, WarcWriter};

fn main() -> Result<(), std::io::Error> {
    let date = Record::make_date();
    let body = format!("wrote to the file on {}", date);
    let body = body.into_bytes();

    let record = Record {
        version: "1.0".to_owned(),
        headers: vec![
            (WARC_RECORD_ID.to_owned(), Record::make_uuid().into_bytes()),
            (
                WARC_TYPE.to_owned(),
                RecordType::WarcInfo.to_string().into_bytes(),
            ),
            (WARC_DATE.to_owned(), date.into_bytes()),
            (
                WARC_IP_ADDRESS.to_owned(),
                "127.0.0.1".to_owned().into_bytes(),
            ),
            (
                CONTENT_LENGTH.to_owned(),
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
