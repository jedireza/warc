use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum WarcRecordType {
    WarcInfo,
    Response,
    Resource,
    Request,
    Metadata,
    Revisit,
    Conversion,
    Continuation,
    Unknown(String),
}

impl fmt::Display for WarcRecordType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            WarcRecordType::WarcInfo => "warcinfo",
            WarcRecordType::Response => "response",
            WarcRecordType::Resource => "resource",
            WarcRecordType::Request => "request",
            WarcRecordType::Metadata => "metadata",
            WarcRecordType::Revisit => "revisit",
            WarcRecordType::Conversion => "conversion",
            WarcRecordType::Continuation => "continuation",
            WarcRecordType::Unknown(ref val) => val.as_ref(),
        })
    }
}
