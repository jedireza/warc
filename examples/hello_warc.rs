use warc::header::{WARC_DATE, WARC_IP_ADDRESS, WARC_RECORD_ID, WARC_TYPE};
use warc::{WarcRecord, WarcRecordType};

fn main() {
    let id = WarcRecord::make_uuid();
    let warc_type = WarcRecordType::WarcInfo.to_string();
    let date = WarcRecord::make_date();

    let record = WarcRecord {
        version: b"1.0",
        headers: vec![
            (WARC_RECORD_ID, id.as_bytes()),
            (WARC_TYPE, warc_type.as_bytes()),
            (WARC_IP_ADDRESS, b"127.0.0.1"),
            (WARC_DATE, date.as_bytes()),
        ],
        body: "hello warc! 👋".as_bytes(),
    };

    print!("{}", record);
}
