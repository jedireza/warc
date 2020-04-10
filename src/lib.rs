//! A WARC (Web ARChive) library

pub mod header;
pub use header::{WarcHeader, WarcHeaders};

pub mod parser;

mod record;
pub use record::WarcRecord;

mod record_type;
pub use record_type::WarcRecordType;

mod truncated_type;
pub use truncated_type::WarcTruncatedType;
