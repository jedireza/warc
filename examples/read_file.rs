use warc::WarcFile;

fn main() -> Result<(), std::io::Error> {
    let file = WarcFile::open("warc_example.warc")?;

    let mut count = 0;
    for record in file {
        count += 1;
        match record {
            Err(err) => println!("ERROR: {}\r\n", err),
            Ok(record) => print!("{}", record),
        }
    }
    println!("Total records: {}", count);

    Ok(())
}
