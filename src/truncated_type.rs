use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum TruncatedType {
    Length,
    Time,
    Disconnect,
    Unspecified,
    Unknown(String),
}

impl fmt::Display for TruncatedType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            TruncatedType::Length => "length",
            TruncatedType::Time => "time",
            TruncatedType::Disconnect => "disconnect",
            TruncatedType::Unspecified => "unspecified",
            TruncatedType::Unknown(ref val) => val.as_ref(),
        })
    }
}
