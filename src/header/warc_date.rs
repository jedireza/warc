extern crate hyper;

use chrono::{DateTime, UTC};
use chrono::offset::TimeZone;
use std::fmt;
use std::str;
use hyper::header::{Header, HeaderFormat};
use hyper::header::parsing::from_one_raw_str;

/// `WARC-Date` header, defined in ISO28500; section 5.4
///
/// A 14-digit UTC timestamp formatted according to YYYY-MM-DDThh:mm:ssZ,
/// described in the W3C profile of ISO8601 [W3CDTF]. The timestamp shall
/// represent the instant that data capture for record creation began. Multiple
/// records written as part of a single capture event (see section 5.7) shall
/// use the same `WARC-Date`, even though the times of their writing will not be
/// exactly synchronized.
///
/// # ABNF
/// ```plain
/// WARC-Date   = "WARC-Date" ":" w3c-iso8601
/// w3c-iso8601 = <YYYY-MM-DDThh:mm:ssZ>
/// ```
///
/// All records shall have a `WARC-Date` field.
#[derive(Clone, Debug, PartialEq)]
pub struct WARCDate(pub DateTime<UTC>);

impl Header for WARCDate {
    fn header_name() -> &'static str {
        "WARC-Date"
    }

    fn parse_header(raw: &[Vec<u8>]) -> hyper::error::Result<WARCDate> {
        from_one_raw_str(raw).and_then(|val: String| {
            match UTC.datetime_from_str(&val, "%Y-%m-%dT%H:%M:%SZ") {
                Ok(date) => Ok(WARCDate(date)),
                _ => Err(hyper::error::Error::Header)
            }
        })
    }
}

impl HeaderFormat for WARCDate {
    fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%dT%H:%M:%SZ"))
    }
}

impl fmt::Display for WARCDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_header(f)
    }
}

impl str::FromStr for WARCDate {
    type Err = hyper::error::Error;

    fn from_str(val: &str) -> hyper::error::Result<WARCDate> {
        match UTC.datetime_from_str(&val, "%Y-%m-%dT%H:%M:%SZ") {
            Ok(date) => Ok(WARCDate(date)),
            _ => Err(hyper::error::Error::Header)
        }
    }
}
