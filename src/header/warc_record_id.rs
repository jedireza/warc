extern crate hyper;

use std::fmt;
use hyper::header::{Header, HeaderFormat};
use hyper::header::parsing::from_one_raw_str;

/// `WARC-Record-ID` header, defined in ISO28500; section 5.2
///
/// An identifier assigned to the current record that is globally unique for its
/// period of intended use. No identifier scheme is mandated by this
/// specification, but each record-id shall be a legal URI and clearly indicate
/// a documented and registered scheme to which it conforms (e.g., via a URI
/// scheme prefix such as "http:" or "urn:"). Care should be taken to ensure
/// that this value is written with no internal whitespace.
///
/// # ABNF
/// ```plain
/// WARC-Record-ID = "WARC-Record-ID" ":" uri
/// ```
///
/// All records shall have a `WARC-Record-ID` field.
#[derive(Clone, Debug, PartialEq)]
pub struct WARCRecordID(pub String);

impl Header for WARCRecordID {
    fn header_name() -> &'static str {
        "WARC-Record-ID"
    }

    fn parse_header(raw: &[Vec<u8>]) -> hyper::error::Result<WARCRecordID> {
        from_one_raw_str(raw).and_then(|val: String| {
            Ok(WARCRecordID(val))
        })
    }
}

impl HeaderFormat for WARCRecordID {
    fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for WARCRecordID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_header(f)
    }
}
