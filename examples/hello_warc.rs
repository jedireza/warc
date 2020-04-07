use chrono::Utc;
use warc::header::{WARC_DATE, WARC_IP_ADDRESS, WARC_RECORD_ID, WARC_TYPE};
use warc::{WarcRecord, WarcRecordType};

fn main() {
    let mut record = WarcRecord {
        version: "1.0",
        headers: vec![],
        body: "hello warc! 👋".as_bytes(),
    };

    let id = WarcRecord::uuid();
    let warc_type = WarcRecordType::WarcInfo.to_string();
    let date = Utc::now().to_string();

    record.headers.push((WARC_RECORD_ID, id.as_str()));
    record.headers.push((WARC_TYPE, warc_type.as_str()));
    record.headers.push((WARC_IP_ADDRESS, "127.0.0.1"));
    record.headers.push((WARC_DATE, date.as_str()));

    println!("{}", record);
}
