use warc::header::WarcHeader;
use warc::WarcReader;

fn main() -> Result<(), std::io::Error> {
    let file = WarcReader::from_path("warc_example.warc")?;

    let mut count = 0;
    for record in file.iter_records() {
        count += 1;
        match record {
            Err(err) => println!("ERROR: {}\r\n", err),
            Ok(record) => {
                println!("{}: {}", WarcHeader::RecordID.to_string(), record.warc_id(),);
                println!("{}: {}", WarcHeader::Date.to_string(), record.date(),);
                println!("");
            }
        }
    }

    println!("Total records: {}", count);

    Ok(())
}
