use chrono::prelude::*;

use warc::{BufferedBody, Record, RecordType, WarcWriter};

fn main() -> Result<(), std::io::Error> {
    let date = Utc::now();
    let body = format!("wrote to the file on {}", date);
    let body = body.into_bytes();

    let mut headers = Record::<BufferedBody>::new();
    headers.set_warc_type(RecordType::WarcInfo);
    headers.set_date(date);
    headers.set_ip_address("127.0.0.1");
    let record = headers.add_body(body);

    let mut file = WarcWriter::from_path("warc_example.warc")?;

    let bytes_written = file.write(&record)?;

    println!("{} bytes written.", bytes_written);

    Ok(())
}
