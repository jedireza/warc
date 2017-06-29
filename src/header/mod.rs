//! WARC named fields, defined in ISO28500; section 5.1
//!
//! Named fields within a WARC record provide information about the current
//! record, and allow additional per- record information. WARC both reuses
//! appropriate headers from other standards and defines new headers, all
//! beginning `WARC-`, for WARC-specific purposes. WARC named fields of the same
//! type shall not be repeated in the same WARC record (for example, a WARC
//! record shall not have several `WARC-Date` or several `WARC-Target-URI`),
//! except as noted (e.g., `WARC-Concurrent-To`). Because new fields may be
//! defined in extensions to the core WARC format, WARC processing software
//! shall ignore fields with unrecognized names.
//!
//! The `ContentLength` and `ContentType` headers are re-exports from
//! [`hyper::header`](http://hyper.rs/hyper/hyper/header/index.html).

pub use self::content_length::ContentLength;
pub use self::content_type::ContentType;
pub use self::warc_block_digest::WarcBlockDigest;
pub use self::warc_concurrent_to::WarcConcurrentTo;
pub use self::warc_date::WarcDate;
pub use self::warc_filename::WarcFilename;
pub use self::warc_identified_payload_type::WarcIdentifiedPayloadType;
pub use self::warc_ip_address::WarcIpAddress;
pub use self::warc_payload_digest::WarcPayloadDigest;
pub use self::warc_profile::WarcProfile;
pub use self::warc_record_id::WarcRecordID;
pub use self::warc_refers_to::WarcRefersTo;
pub use self::warc_segment_number::WarcSegmentNumber;
pub use self::warc_segment_origin_id::WarcSegmentOriginID;
pub use self::warc_segment_total_length::WarcSegmentTotalLength;
pub use self::warc_target_uri::WarcTargetURI;
pub use self::warc_truncated::WarcTruncated;
pub use self::warc_truncated::WarcTruncatedType;
pub use self::warc_type::WarcRecordType;
pub use self::warc_type::WarcType;
pub use self::warc_warcinfo_id::WarcWarcinfoID;

mod content_length;
mod content_type;
mod warc_block_digest;
mod warc_concurrent_to;
mod warc_date;
mod warc_filename;
mod warc_identified_payload_type;
mod warc_ip_address;
mod warc_payload_digest;
mod warc_profile;
mod warc_record_id;
mod warc_refers_to;
mod warc_segment_number;
mod warc_segment_origin_id;
mod warc_segment_total_length;
mod warc_target_uri;
mod warc_truncated;
mod warc_type;
mod warc_warcinfo_id;
