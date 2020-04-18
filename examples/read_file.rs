use warc::WarcFile;

fn main() -> Result<(), std::io::Error> {
    let file = WarcFile::open("warc_example.warc")?;

    for record in file {
        match record {
            Err(err) => println!("ERROR: {}\r\n", err),
            Ok(record) => print!("{}", record),
        }
    }

    Ok(())
}
