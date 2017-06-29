//! WARC Versions enum

use hyper::error::Error;
use std::fmt;
use std::str::FromStr;

pub enum WarcVersion {
    /// `WARC/1.0`
    Warc10,
    /// `Outputs the value provided`
    Unknown(String),
}

impl fmt::Display for WarcVersion {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match *self {
            WarcVersion::Warc10 => "WARC/1.0",
            WarcVersion::Unknown(ref val) => val.as_ref(),
        })
    }
}

impl FromStr for WarcVersion {
    type Err = Error;

    fn from_str(val: &str) -> Result<WarcVersion, Error> {
        Ok(match val {
            "WARC/1.0" => WarcVersion::Warc10,
            _ => WarcVersion::Unknown(val.to_owned()),
        })
    }
}
