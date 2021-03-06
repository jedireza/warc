use warc::header::WarcHeader;
use warc::WarcReader;

fn main() -> Result<(), std::io::Error> {
    let file = WarcReader::from_path("warc_example.warc")?;

    let mut count = 0;
    for record in file.iter_raw_records() {
        count += 1;
        match record {
            Err(err) => println!("ERROR: {}\r\n", err),
            Ok((headers, _)) => {
                println!(
                    "{}: {}",
                    WarcHeader::RecordID.to_string(),
                    String::from_utf8_lossy(headers.as_ref().get(&WarcHeader::RecordID).unwrap())
                );
                println!(
                    "{}: {}",
                    WarcHeader::Date.to_string(),
                    String::from_utf8_lossy(headers.as_ref().get(&WarcHeader::Date).unwrap())
                );
                println!("");
            }
        }
    }

    println!("Total records: {}", count);

    Ok(())
}
