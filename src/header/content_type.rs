//! `Content-Type` header, defined in ISO28500; section 5.6
//!
//! The MIME type [RFC2045] of the information contained in the record's block.
//! For example, in HTTP request and response records, this would be
//! `application/http` as per section 19.1 of [RFC2616] (or `application/http;
//! msgtype=request` and `application/http; msgtype=response` respectively). In
//! particular, the content-type is not the value of the HTTP Content-Type
//! header in an HTTP response but a MIME type to describe the full archived
//! HTTP message (hence `application/http` if the block contains request or
//! response headers).
//!
//! # ABNF
//! ```plain
//! Content-Type   = "Content-Type" ":" media-type
//! media-type     = type "/" subtype *( ";" parameter )
//! type           = token
//! subtype        = token
//! parameter      = attribute "=" value
//! attribute      = token
//! value          = token | quoted-string
//! ```
//!
//! All records with a non-empty block (non-zero Content-Length), except
//! `continuation` records, should have a Content-Type field. Only if the media
//! type is not given by a Content-Type field, a reader may attempt to guess the
//! media type via inspection of its content and/or the name extension(s) of the
//! URI used to identify the resource. If the media type remains unknown, the
//! reader should treat it as type `application/octet-stream`.

extern crate hyper;

pub use hyper::header::ContentType;
