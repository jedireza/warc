use warc::WarcFile;

fn main() -> Result<(), std::io::Error> {
    let file = WarcFile::open("warc_example.warc")?;

    for record in file {
        print!("{}", record.unwrap());
    }

    Ok(())
}
