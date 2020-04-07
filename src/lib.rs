//! A WARC (Web ARChive) library

pub mod header;
pub mod parser;
mod record;
mod record_type;
mod truncated_type;
mod version;

pub use record::WarcRecord;
pub use record_type::WarcRecordType;
pub use truncated_type::WarcTruncatedType;
pub use version::WarcVersion;
