extern crate hyper;

use hyper::header::{Header, HeaderFormat};
use hyper::header::parsing::from_one_raw_str;
use std::fmt;
use std::str;
use std::str::FromStr;

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
pub struct WARCTruncated(pub WARCTruncatedType);

impl Header for WARCTruncated {
    fn header_name() -> &'static str {
        "WARC-Truncated"
    }

    fn parse_header(raw: &[Vec<u8>]) -> hyper::error::Result<WARCTruncated> {
        from_one_raw_str(raw).and_then(|val: String| {
            WARCTruncated::from_str(&val)
        })
    }
}

impl HeaderFormat for WARCTruncated {
    fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for WARCTruncated {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_header(f)
    }
}

impl str::FromStr for WARCTruncated {
    type Err = hyper::error::Error;

    fn from_str(val: &str) -> hyper::error::Result<WARCTruncated> {
        match val {
            "length" => Ok(WARCTruncated(WARCTruncatedType::Length)),
            "time" => Ok(WARCTruncated(WARCTruncatedType::Time)),
            "disconnect" => Ok(WARCTruncated(WARCTruncatedType::Disconnect)),
            "unspecified" => Ok(WARCTruncated(WARCTruncatedType::Unspecified)),
            _ => Ok(WARCTruncated(WARCTruncatedType::Unknown(val.to_owned()))),
        }
    }
}

/// Variants for `WARCTruncated` header
///
/// As defined in ISO28500; section 5.13
#[derive(Clone, Debug, PartialEq)]
pub enum WARCTruncatedType {
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

impl fmt::Display for WARCTruncatedType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            WARCTruncatedType::Length => "length",
            WARCTruncatedType::Time => "time",
            WARCTruncatedType::Disconnect => "disconnect",
            WARCTruncatedType::Unspecified => "unspecified",
            WARCTruncatedType::Unknown(ref val) => val.as_ref(),
        })
    }
}
