#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum WarcHeader {
    CONTENT_LENGTH,
    CONTENT_TYPE,
    WARC_BLOCK_DIGEST,
    WARC_CONCURRENT_TO,
    WARC_DATE,
    WARC_FILENAME,
    WARC_IDENTIFIED_PAYLOAD_TYPE,
    WARC_IP_ADDRESS,
    WARC_PAYLOAD_DIGEST,
    WARC_PROFILE,
    WARC_RECORD_ID,
    WARC_REFERS_TO,
    WARC_SEGMENT_NUMBER,
    WARC_SEGMENT_ORIGIN_ID,
    WARC_SEGMENT_TOTAL_LENGTH,
    WARC_TARGET_URI,
    WARC_TRUNCATED,
    WARC_TYPE,
    WARC_WARCINFO_ID,
    Unknown(String),
}

impl ToString for WarcHeader {
    fn to_string(&self) -> String {
        let stringified = match self {
            &WarcHeader::CONTENT_LENGTH => "content-length",
            &WarcHeader::CONTENT_TYPE => "content-type",
            &WarcHeader::WARC_BLOCK_DIGEST => "warc-block-digest",
            &WarcHeader::WARC_CONCURRENT_TO => "warc-concurrent-to",
            &WarcHeader::WARC_DATE => "warc-date",
            &WarcHeader::WARC_FILENAME => "warc-filename",
            &WarcHeader::WARC_IDENTIFIED_PAYLOAD_TYPE => "warc-identified-payload-type",
            &WarcHeader::WARC_IP_ADDRESS => "warc-ip-address",
            &WarcHeader::WARC_PAYLOAD_DIGEST => "warc-payload-digest",
            &WarcHeader::WARC_PROFILE => "warc-profile",
            &WarcHeader::WARC_RECORD_ID => "warc-record-id",
            &WarcHeader::WARC_REFERS_TO => "warc-refers-to",
            &WarcHeader::WARC_SEGMENT_NUMBER => "warc-segment-number",
            &WarcHeader::WARC_SEGMENT_ORIGIN_ID => "warc-segment-origin-id",
            &WarcHeader::WARC_SEGMENT_TOTAL_LENGTH => "warc-segment-total-length",
            &WarcHeader::WARC_TARGET_URI => "warc-target-uri",
            &WarcHeader::WARC_TRUNCATED => "warc-truncated",
            &WarcHeader::WARC_TYPE => "warc-type",
            &WarcHeader::WARC_WARCINFO_ID => "warc-warcinfo-id",
            &WarcHeader::Unknown(ref string) => string,
        };
        stringified.to_string()
    }
}

impl<S: AsRef<str>> From<S> for WarcHeader {
    fn from(string: S) -> Self {
        let lower: String = string.as_ref().to_lowercase();
        match lower.as_str() {
            "content-length" => WarcHeader::CONTENT_LENGTH,
            "content-type" => WarcHeader::CONTENT_TYPE,
            "warc-block-digest" => WarcHeader::WARC_BLOCK_DIGEST,
            "warc-concurrent-to" => WarcHeader::WARC_CONCURRENT_TO,
            "warc-date" => WarcHeader::WARC_DATE,
            "warc-filename" => WarcHeader::WARC_FILENAME,
            "warc-identified-payload-type" => WarcHeader::WARC_IDENTIFIED_PAYLOAD_TYPE,
            "warc-ip-address" => WarcHeader::WARC_IP_ADDRESS,
            "warc-payload-digest" => WarcHeader::WARC_PAYLOAD_DIGEST,
            "warc-profile" => WarcHeader::WARC_PROFILE,
            "warc-record-id" => WarcHeader::WARC_RECORD_ID,
            "warc-refers-to" => WarcHeader::WARC_REFERS_TO,
            "warc-segment-number" => WarcHeader::WARC_SEGMENT_NUMBER,
            "warc-segment-origin-id" => WarcHeader::WARC_SEGMENT_ORIGIN_ID,
            "warc-segment-total-length" => WarcHeader::WARC_SEGMENT_TOTAL_LENGTH,
            "warc-target-uri" => WarcHeader::WARC_TARGET_URI,
            "warc-truncated" => WarcHeader::WARC_TRUNCATED,
            "warc-type" => WarcHeader::WARC_TYPE,
            "warc-warcinfo-id" => WarcHeader::WARC_WARCINFO_ID,
            _ => WarcHeader::Unknown(lower),
        }
    }
}
