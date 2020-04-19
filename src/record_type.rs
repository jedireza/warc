use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum RecordType {
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

impl fmt::Display for RecordType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            RecordType::WarcInfo => "warcinfo",
            RecordType::Response => "response",
            RecordType::Resource => "resource",
            RecordType::Request => "request",
            RecordType::Metadata => "metadata",
            RecordType::Revisit => "revisit",
            RecordType::Conversion => "conversion",
            RecordType::Continuation => "continuation",
            RecordType::Unknown(ref val) => val.as_ref(),
        })
    }
}
