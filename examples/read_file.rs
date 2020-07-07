use warc::header::WarcHeader;
use warc::WarcReader;

fn main() -> Result<(), std::io::Error> {
    let file = WarcReader::from_path("warc_example.warc")?;

    let mut count = 0;
    for record in file {
        count += 1;
        match record {
            Err(err) => println!("ERROR: {}\r\n", err),
            Ok(record) => {
                println!(
                    "{}: {}",
                    WarcHeader::WARC_RECORD_ID.to_string(),
                    String::from_utf8_lossy(record.headers.get(&WarcHeader::WARC_RECORD_ID).unwrap())
                );
                println!(
                    "{}: {}",
                    WarcHeader::WARC_DATE.to_string(),
                    String::from_utf8_lossy(record.headers.get(&WarcHeader::WARC_DATE).unwrap())
                );
                println!("");
            }
        }
    }

    println!("Total records: {}", count);

    Ok(())
}
