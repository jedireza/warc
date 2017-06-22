extern crate warc;

use warc::WarcRecord;
use warc::header::WarcIpAddress;
use warc::header::WarcRecordType;
use warc::header::WarcType;

fn main() {
    let mut rec = WarcRecord::new();

    rec.headers.set(WarcType(WarcRecordType::WarcInfo));
    rec.headers.set(WarcIpAddress("127.0.0.1".to_owned()));
    rec.set_body("hello world! 👋".to_owned().into_bytes());

    println!("{}", rec);
}
