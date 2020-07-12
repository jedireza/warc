use warc::header::WarcHeader;
use warc::{Record, RecordType, WarcWriter};

fn main() -> Result<(), std::io::Error> {
    let date = Record::make_date();
    let body = format!("wrote to the file on {}", date);
    let body = body.into_bytes();

    let record = Record {
        version: "1.0".to_owned(),
        headers: vec![
            (WarcHeader::RecordID.to_owned(), Record::make_uuid().into_bytes()),
            (
                WarcHeader::WarcType.to_owned(),
                RecordType::WarcInfo.to_string().into_bytes(),
            ),
            (WarcHeader::Date.to_owned(), date.into_bytes()),
            (
                WarcHeader::IPAddress.to_owned(),
                "127.0.0.1".to_owned().into_bytes(),
            ),
            (
                WarcHeader::ContentLength.to_owned(),
                body.len().to_string().into_bytes(),
            ),
        ]
        .into_iter()
        .collect(),
        body: body,
    };

    let mut file = WarcWriter::from_path_gzip("warc_example.warc.gz")?;

    let bytes_written = file.write(&record)?;

    // NB: the compression stream must be finish()ed, or the file will be truncated
    let gzip_stream = file.into_inner()?;
    gzip_stream.finish().into_result()?;

    println!("{} bytes written.", bytes_written);

    Ok(())
}
