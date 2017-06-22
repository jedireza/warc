extern crate hyper;

use chrono::DateTime;
use chrono::Utc;
use chrono::offset::TimeZone;
use hyper::header::Formatter;
use hyper::header::Header;
use hyper::header::Raw;
use hyper::header::parsing::from_one_raw_str;
use std::fmt;
use std::str;

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
pub struct WarcDate(pub DateTime<Utc>);

impl Header for WarcDate {
    fn header_name() -> &'static str {
        "WARC-Date"
    }

    fn parse_header(raw: &Raw) -> hyper::error::Result<WarcDate> {
        from_one_raw_str(raw).and_then(|val: String| {
            match Utc.datetime_from_str(&val, "%Y-%m-%dT%H:%M:%SZ") {
                Ok(date) => Ok(WarcDate(date)),
                _ => Err(hyper::error::Error::Header)
            }
        })
    }

    fn fmt_header(&self, f: &mut Formatter) -> fmt::Result {
        f.fmt_line(&self.0.format("%Y-%m-%dT%H:%M:%SZ"))
    }
}

impl str::FromStr for WarcDate {
    type Err = hyper::error::Error;

    fn from_str(val: &str) -> hyper::error::Result<WarcDate> {
        match Utc.datetime_from_str(&val, "%Y-%m-%dT%H:%M:%SZ") {
            Ok(date) => Ok(WarcDate(date)),
            _ => Err(hyper::error::Error::Header)
        }
    }
}
