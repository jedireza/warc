#![allow(missing_docs)]
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

impl ToString for RecordType {
    fn to_string(&self) -> String {
        let stringified = match *self {
            RecordType::WarcInfo => "warcinfo",
            RecordType::Response => "response",
            RecordType::Resource => "resource",
            RecordType::Request => "request",
            RecordType::Metadata => "metadata",
            RecordType::Revisit => "revisit",
            RecordType::Conversion => "conversion",
            RecordType::Continuation => "continuation",
            RecordType::Unknown(ref val) => val.as_ref(),
        };
        stringified.to_string()
    }
}

impl<S: AsRef<str>> From<S> for RecordType {
    fn from(string: S) -> Self {
        let lower: String = string.as_ref().to_lowercase();
        match lower.as_str() {
            "warcinfo" => RecordType::WarcInfo,
            "response" => RecordType::Response,
            "resource" => RecordType::Resource,
            "request" => RecordType::Request,
            "metadata" => RecordType::Metadata,
            "revisit" => RecordType::Revisit,
            "conversion" => RecordType::Conversion,
            "continuation" => RecordType::Continuation,
            _ => RecordType::Unknown(lower),
        }
    }
}
