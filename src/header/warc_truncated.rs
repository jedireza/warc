extern crate hyper;

use hyper::header::Formatter;
use hyper::header::Header;
use hyper::header::Raw;
use hyper::header::parsing::from_one_raw_str;
use std::fmt;
use std::str::FromStr;
use std::str;

/// `WARC-Truncated` header, defined in ISO28500; section 5.13
///
/// For practical reasons, writers of the WARC format may place limits on the
/// time or storage allocated to archiving a single resource. As a result, only
/// a truncated portion of the original resource may be available for saving
/// into a WARC record.
///
/// Any record may indicate that truncation of its content block has occurred
/// and give the reason with a `WARC-Truncated` field.
///
/// # ABNF
/// ```plain
/// WARC-Truncated  = "WARC-Truncated" ":" reason-token
/// reason-token    = "length"         ; exceeds configured max length
///                 | "time"           ; exceeds configured max time
///                 | "disconnect"     ; network disconnect
///                 | "unspecified"    ; other/unknown reason
///                 | future-reason
/// future-reason   = token
/// ```
///
/// For example, if the capture of what appeared to be a multi-gigabyte resource
/// was cut short after a transfer time limit was reached, the partial resource
/// could be saved to a WARC record with this field.
///
/// The `WARC-Truncated` field may be used on any WARC record. The WARC field
/// `Content-Length` shall still report the actual truncated size of the record
/// block.
#[derive(Clone, Debug, PartialEq)]
pub struct WarcTruncated(pub WarcTruncatedType);

impl Header for WarcTruncated {
    fn header_name() -> &'static str {
        "WARC-Truncated"
    }

    fn parse_header(raw: &Raw) -> hyper::error::Result<WarcTruncated> {
        from_one_raw_str(raw).and_then(|val: String| {
            WarcTruncated::from_str(&val)
        })
    }

    fn fmt_header(&self, f: &mut Formatter) -> fmt::Result {
        f.fmt_line(&self.0)
    }
}

impl str::FromStr for WarcTruncated {
    type Err = hyper::error::Error;

    fn from_str(val: &str) -> hyper::error::Result<WarcTruncated> {
        match val {
            "length" => Ok(WarcTruncated(WarcTruncatedType::Length)),
            "time" => Ok(WarcTruncated(WarcTruncatedType::Time)),
            "disconnect" => Ok(WarcTruncated(WarcTruncatedType::Disconnect)),
            "unspecified" => Ok(WarcTruncated(WarcTruncatedType::Unspecified)),
            _ => Ok(WarcTruncated(WarcTruncatedType::Unknown(val.to_owned()))),
        }
    }
}

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
