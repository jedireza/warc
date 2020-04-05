extern crate warc;

use warc::header::WARC_IP_ADDRESS;
use warc::WarcRecord;

fn main() {
    let mut record = WarcRecord::new();

    // rec.headers.set(WarcType(WarcRecordType::WarcInfo));
    record
        .headers
        .insert(WARC_IP_ADDRESS, "127.0.0.1".parse().unwrap());
    record.set_body("hello warc! 👋".to_owned().into_bytes());

    println!("{}", record);
}
