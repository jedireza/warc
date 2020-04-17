use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum WarcTruncatedType {
    Length,
    Time,
    Disconnect,
    Unspecified,
    Unknown(String),
}

impl fmt::Display for WarcTruncatedType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            WarcTruncatedType::Length => "length",
            WarcTruncatedType::Time => "time",
            WarcTruncatedType::Disconnect => "disconnect",
            WarcTruncatedType::Unspecified => "unspecified",
            WarcTruncatedType::Unknown(ref val) => val.as_ref(),
        })
    }
}
