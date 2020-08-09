use std::error;
use std::fmt;

use crate::header::WarcHeader;

/// An error type returned by WARC header parsing.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// An error occured identifing or parsing headers.
    ParseHeaders,
    /// A header required by the standard is missing from the record. The record was well-formed,
    /// but invalid.
    MissingHeader(WarcHeader),
    /// The underlying read from the data source failed.
    ReadData,
    /// More data was read than expected by the header metadata. The record was well-formed, but
    /// invalid.
    ReadOverflow,
    /// The end of the record's body was found unexpectedly.
    UnexpectedEOB,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ParseHeaders => write!(f, "Error parsing headers."),
            Error::MissingHeader(ref h) => write!(f, "Missing required header: {}", h.to_string()),
            Error::ReadData => write!(f, "Error reading data source."),
            Error::ReadOverflow => write!(f, "Read further than expected."),
            Error::UnexpectedEOB => write!(f, "Unexpected end of body."),
        }
    }
}

impl error::Error for Error {}
