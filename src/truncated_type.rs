use std::fmt;

/// Variants for `WARCTruncated` header
///
/// As defined in ISO28500; section 5.13
#[derive(Clone, Debug, PartialEq)]
pub enum WarcTruncatedType {
    /// `length` exceeds configured max length
    Length,
    /// `time` exceeds configured max time
    Time,
    /// `disconnect` network disconnect
    Disconnect,
    /// `unspecified` other/unknown reason
    Unspecified,
    /// future reason
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
