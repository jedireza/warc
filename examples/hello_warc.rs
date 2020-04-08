use warc::header::{WARC_DATE, WARC_IP_ADDRESS, WARC_RECORD_ID, WARC_TYPE};
use warc::{WarcRecord, WarcRecordType};

fn main() {
    let id = WarcRecord::uuid();
    let warc_type = WarcRecordType::WarcInfo.to_string();
    let ip_addr = "127.0.0.1".to_owned();
    let date = WarcRecord::now();

    let record = WarcRecord {
        version: "1.0",
        headers: vec![
            (WARC_RECORD_ID, id.as_str()),
            (WARC_TYPE, warc_type.as_str()),
            (WARC_IP_ADDRESS, ip_addr.as_str()),
            (WARC_DATE, date.as_str()),
        ],
        body: "hello warc! 👋".as_bytes(),
    };

    println!("{}", record);
}
