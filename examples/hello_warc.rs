extern crate warc;

use warc::WARCRecord;

fn main() {
    let rec = WARCRecord::new();

    println!("{}", rec);
}
