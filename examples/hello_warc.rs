use warc::header::{WARC_DATE, WARC_IP_ADDRESS, WARC_RECORD_ID, WARC_TYPE};
use warc::{WarcHeader, WarcRecord, WarcRecordType};

fn main() {
    let record = WarcRecord {
        version: "1.0".to_owned(),
        headers: vec![
            WarcHeader::new(WARC_RECORD_ID, WarcRecord::make_uuid().to_owned()),
            WarcHeader::new(WARC_TYPE, WarcRecordType::WarcInfo.to_string()),
            WarcHeader::new(WARC_DATE, WarcRecord::make_date()),
            WarcHeader::new(WARC_IP_ADDRESS, "127.0.0.1".to_owned()),
        ],
        body: "hello warc! 👋".to_owned().into_bytes(),
    };

    print!("{}", record);
}
