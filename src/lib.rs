//! A WARC (Web ARChive) library

mod error;
pub use error::Error;

pub mod warc_types;
pub use warc_types::{WarcReader, WarcWriter};

pub mod header;

pub mod parser;

mod record;
pub use record::Record;

mod record_type;
pub use record_type::RecordType;

mod truncated_type;
pub use truncated_type::TruncatedType;
