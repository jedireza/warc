use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    ParseHeaders,
    ReadData,
    ReadOverflow,
    UnexpectedEOB,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ParseHeaders => write!(f, "Error parsing headers."),
            Error::ReadData => write!(f, "Error reading data source."),
            Error::ReadOverflow => write!(f, "Read further than expected."),
            Error::UnexpectedEOB => write!(f, "Unexpected end of body."),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::ParseHeaders => None,
            Error::ReadData => None,
            Error::ReadOverflow => None,
            Error::UnexpectedEOB => None,
        }
    }
}
