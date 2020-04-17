//! A WARC (Web ARChive) library

pub mod error;
pub use error::WarcError;

mod file;
pub use file::WarcFile;

pub mod header;
pub use header::{WarcHeader, WarcHeaderRef, WarcHeaders, WarcHeadersRef};

pub mod parser;

mod record;
pub use record::{WarcRecord, WarcRecordRef};

mod record_type;
pub use record_type::WarcRecordType;

mod truncated_type;
pub use truncated_type::WarcTruncatedType;
