use std::error;
use std::fmt;

#[derive(Debug)]
pub enum WarcError {
    ParseHeaders,
    ReadData,
    EmptyRead,
}

impl fmt::Display for WarcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WarcError::ParseHeaders => write!(f, "Error parsing headers."),
            WarcError::ReadData => write!(f, "Error reading data source."),
            WarcError::EmptyRead => write!(f, "Unexpected empty read."),
        }
    }
}

impl error::Error for WarcError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            WarcError::ParseHeaders => None,
            WarcError::ReadData => None,
            WarcError::EmptyRead => None,
        }
    }
}
