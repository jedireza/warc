use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    ParseHeaders,
    ReadData,
    UnexpectedEOB,
    ReadOverflow,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ParseHeaders => write!(f, "Error parsing headers."),
            Error::ReadData => write!(f, "Error reading data source."),
            Error::UnexpectedEOB => write!(f, "Unexpected end of body."),
            Error::ReadOverflow => write!(f, "Read further than expected."),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::ParseHeaders => None,
            Error::ReadData => None,
            Error::UnexpectedEOB => None,
            Error::ReadOverflow => None,
        }
    }
}
