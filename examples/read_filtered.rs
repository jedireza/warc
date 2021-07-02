use warc::header::WarcHeader;
use warc::WarcReader;

macro_rules! usage_err {
    ($str:expr) => {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, $str.to_string())
    };
}

fn main() -> std::io::Result<()> {
    let mut args = std::env::args_os().skip(1);

    let warc_name = args
        .next()
        .ok_or_else(|| usage_err!("compressed warc filename not supplied"))?;

    let filtered_file_names: Vec<_> = args.map(|s| s.to_string_lossy().to_string()).collect();
    if filtered_file_names.is_empty() {
        return Err(usage_err!("one or more filtered file names not supplied"))?;
    }

    let mut file = WarcReader::from_path_gzip(warc_name)?;

    let mut count = 0;
    let mut skipped = 0;
    let mut stream_iter = file.stream_records();
    while let Some(record) = stream_iter.next_item() {
        let record = record.expect("read of headers ok");
        count += 1;
        match record.header(WarcHeader::TargetURI).map(|s| s.to_string()) {
            Some(v) if has_matching_filename(&v, &filtered_file_names) => {
                println!("Matches filename, skipping record");
                skipped += 1;
            }
            _ => {
                let buffered = record.into_buffered().expect("read of record ok");
                println!(
                    "Found record. Data:\n{}",
                    String::from_utf8_lossy(buffered.body())
                );
            }
        }
    }

    println!("Total records: {}\nSkipped records: {}", count, skipped);

    Ok(())
}

fn has_matching_filename(u: &str, matches: &[String]) -> bool {
    let url = url::Url::parse(u).expect("Target URI is not a URI!?");
    let iter = match url.path_segments() {
        None => return false,
        Some(it) => it,
    };
    let last_segment = match iter.last() {
        None => return false,
        Some(s) => s.to_string(),
    };
    matches.contains(&last_segment)
}
