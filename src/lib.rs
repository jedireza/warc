//! A WARC (Web ARChive) library

extern crate uuid;
extern crate chrono;
extern crate hyper;

pub mod header;
mod warc_record;

pub use warc_record::WARCRecord;
