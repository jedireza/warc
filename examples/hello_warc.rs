use warc::header::{CONTENT_LENGTH, WARC_DATE, WARC_IP_ADDRESS, WARC_RECORD_ID, WARC_TYPE};
use warc::{WarcHeader, WarcHeaders, WarcRecord, WarcRecordType};

fn main() {
    let body = "hello warc! 👋".to_owned().into_bytes();

    let record = WarcRecord {
        version: "1.0".to_owned(),
        headers: WarcHeaders::new(vec![
            WarcHeader::new(WARC_RECORD_ID, WarcRecord::make_uuid().to_owned()),
            WarcHeader::new(WARC_TYPE, WarcRecordType::WarcInfo.to_string()),
            WarcHeader::new(WARC_DATE, WarcRecord::make_date()),
            WarcHeader::new(WARC_IP_ADDRESS, "127.0.0.1".to_owned()),
            WarcHeader::new(CONTENT_LENGTH, body.len().to_string()),
        ]),
        body: body,
    };

    print!("{}", record);
}
