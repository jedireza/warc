use warc::header::{WARC_DATE, WARC_RECORD_ID};
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
                    WARC_RECORD_ID,
                    String::from_utf8_lossy(record.headers.get(WARC_RECORD_ID).unwrap())
                );
                println!(
                    "{}: {}",
                    WARC_DATE,
                    String::from_utf8_lossy(record.headers.get(WARC_DATE).unwrap())
                );
                println!("");
            }
        }
    }

    println!("Total records: {}", count);

    Ok(())
}
