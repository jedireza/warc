extern crate warc;

use warc::WarcRecord;

fn main() {
    let mut warc = WarcRecord::new();

    warc.set_body("hello warc! 👋".to_owned().into_bytes());

    println!("{}", rec);
}
