//! `Content-Length` header, defined in ISO28500; section 5.3
//!
//! The number of octets in the block, similar to [RFC2616]. If no block is
//! present, a value of '0' (zero) shall be used.
//!
//! # ABNF
//! ```plain
//! Content-Length = "Content-Length" ":" 1*DIGIT
//! ```
//!
//! All records shall have a Content-Length field.

extern crate hyper;

pub use hyper::header::ContentLength;
