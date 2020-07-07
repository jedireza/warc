use std::error;
use std::fmt;

use crate::header::WarcHeader;

#[derive(Debug)]
pub enum Error {
    ParseHeaders,
    MissingHeader(WarcHeader),
    ReadData,
    ReadOverflow,
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

impl error::Error for Error { }
