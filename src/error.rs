use std::error;
use std::fmt;

use crate::header::WarcHeader;

/// An error type returned by WARC header parsing.
#[derive(Debug)]
pub enum Error {
    /// An error occured identifing or parsing headers.
    ParseHeaders(nom::Err<(Vec<u8>, nom::error::ErrorKind)>),
    /// A header required by the standard is missing from the record. The record was well-formed,
    /// but invalid.
    MissingHeader(WarcHeader),
    /// A required header is not well-formed according to the standard.
    MalformedHeader(WarcHeader, String),
    /// The underlying read from the data source failed.
    ReadData(std::io::Error),
    /// More data was read than expected by the header metadata. The record was well-formed, but
    /// invalid.
    ReadOverflow,
    /// The end of the record's body was found unexpectedly.
    UnexpectedEOB,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ParseHeaders(_) => write!(f, "Error parsing headers."),
            Error::MissingHeader(ref h) => write!(f, "Missing required header: {}", h.to_string()),
            Error::MalformedHeader(ref h, ref r) => {
                write!(f, "Malformed header: {}: {}", h.to_string(), r)
            }
            Error::ReadData(_) => write!(f, "Error reading data source."),
            Error::ReadOverflow => write!(f, "Read further than expected."),
            Error::UnexpectedEOB => write!(f, "Unexpected end of body."),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ParseHeaders(ref e) => Some(e),
            Error::ReadData(ref e) => Some(e),
            _ => None,
        }
    }
}
