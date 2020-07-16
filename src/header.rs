#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum WarcHeader {
    ContentLength,
    ContentType,
    BlockDigest,
    ConcurrentTo,
    Date,
    Filename,
    IdentifiedPayloadType,
    IPAddress,
    PayloadDigest,
    Profile,
    RecordID,
    RefersTo,
    SegmentNumber,
    SegmentOriginID,
    SegmentTotalLength,
    TargetURI,
    Truncated,
    WarcType,
    WarcInfoID,
    Unknown(String),
}

impl ToString for WarcHeader {
    fn to_string(&self) -> String {
        let stringified = match self {
            WarcHeader::ContentLength => "content-length",
            WarcHeader::ContentType => "content-type",
            WarcHeader::BlockDigest => "warc-block-digest",
            WarcHeader::ConcurrentTo => "warc-concurrent-to",
            WarcHeader::Date => "warc-date",
            WarcHeader::Filename => "warc-filename",
            WarcHeader::IdentifiedPayloadType => "warc-identified-payload-type",
            WarcHeader::IPAddress => "warc-ip-address",
            WarcHeader::PayloadDigest => "warc-payload-digest",
            WarcHeader::Profile => "warc-profile",
            WarcHeader::RecordID => "warc-record-id",
            WarcHeader::RefersTo => "warc-refers-to",
            WarcHeader::SegmentNumber => "warc-segment-number",
            WarcHeader::SegmentOriginID => "warc-segment-origin-id",
            WarcHeader::SegmentTotalLength => "warc-segment-total-length",
            WarcHeader::TargetURI => "warc-target-uri",
            WarcHeader::Truncated => "warc-truncated",
            WarcHeader::WarcType => "warc-type",
            WarcHeader::WarcInfoID => "warc-warcinfo-id",
            WarcHeader::Unknown(ref string) => string,
        };
        stringified.to_string()
    }
}

impl<S: AsRef<str>> From<S> for WarcHeader {
    fn from(string: S) -> Self {
        let lower: String = string.as_ref().to_lowercase();
        match lower.as_str() {
            "content-length" => WarcHeader::ContentLength,
            "content-type" => WarcHeader::ContentType,
            "warc-block-digest" => WarcHeader::BlockDigest,
            "warc-concurrent-to" => WarcHeader::ConcurrentTo,
            "warc-date" => WarcHeader::Date,
            "warc-filename" => WarcHeader::Filename,
            "warc-identified-payload-type" => WarcHeader::IdentifiedPayloadType,
            "warc-ip-address" => WarcHeader::IPAddress,
            "warc-payload-digest" => WarcHeader::PayloadDigest,
            "warc-profile" => WarcHeader::Profile,
            "warc-record-id" => WarcHeader::RecordID,
            "warc-refers-to" => WarcHeader::RefersTo,
            "warc-segment-number" => WarcHeader::SegmentNumber,
            "warc-segment-origin-id" => WarcHeader::SegmentOriginID,
            "warc-segment-total-length" => WarcHeader::SegmentTotalLength,
            "warc-target-uri" => WarcHeader::TargetURI,
            "warc-truncated" => WarcHeader::Truncated,
            "warc-type" => WarcHeader::WarcType,
            "warc-warcinfo-id" => WarcHeader::WarcInfoID,
            _ => WarcHeader::Unknown(lower),
        }
    }
}
