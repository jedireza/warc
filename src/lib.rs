//! A WARC (Web ARChive) library

extern crate uuid;
extern crate chrono;
#[macro_use] extern crate hyper;

pub mod header;
mod warc_record;

pub use warc_record::WARCRecord;
