//! A WARC (Web ARChive) library

pub mod error;
pub use error::Error;

mod file;
pub use file::File;

pub mod header;

pub mod parser;

mod record;
pub use record::Record;

mod record_type;
pub use record_type::RecordType;

mod truncated_type;
pub use truncated_type::TruncatedType;
