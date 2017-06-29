//! A WARC (Web ARChive) library

extern crate chrono;
#[macro_use] extern crate hyper;
extern crate uuid;

pub mod header;
mod warc_record;
mod warc_version;

pub use warc_record::WarcRecord;
pub use warc_version::WarcVersion;
