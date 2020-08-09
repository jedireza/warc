#[derive(Clone, Debug, PartialEq)]
pub enum TruncatedType {
    Length,
    Time,
    Disconnect,
    Unspecified,
    Unknown(String),
}

impl ToString for TruncatedType {
    fn to_string(&self) -> String {
        let stringified = match *self {
            TruncatedType::Length => "length",
            TruncatedType::Time => "time",
            TruncatedType::Disconnect => "disconnect",
            TruncatedType::Unspecified => "unspecified",
            TruncatedType::Unknown(ref val) => val.as_ref(),
        };
        stringified.to_string()
    }
}

impl<S: AsRef<str>> From<S> for TruncatedType {
    fn from(string: S) -> Self {
        let lower: String = string.as_ref().to_lowercase();
        match lower.as_str() {
            "length" => TruncatedType::Length,
            "time" => TruncatedType::Time,
            "disconnect" => TruncatedType::Disconnect,
            "unspecified" => TruncatedType::Unspecified,
            _ => TruncatedType::Unknown(lower),
        }
    }
}
