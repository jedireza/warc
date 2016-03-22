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

pub use self::warc_record_id::WARCRecordID;
pub use self::warc_date::WARCDate;
pub use self::warc_type::{WARCType, WARCRecordType};
pub use self::content_length::ContentLength;
pub use self::content_type::ContentType;
pub use self::warc_concurrent_to::WARCConcurrentTo;
pub use self::warc_block_digest::WARCBlockDigest;
pub use self::warc_payload_digest::WARCPayloadDigest;
pub use self::warc_ip_address::WARCIPAddress;
pub use self::warc_refers_to::WARCRefersTo;
pub use self::warc_target_uri::WARCTargetURI;
pub use self::warc_truncated::WARCTruncated;
pub use self::warc_warcinfo_id::WARCWarcinfoID;
pub use self::warc_filename::WARCFilename;
pub use self::warc_profile::WARCProfile;
pub use self::warc_identified_payload_type::WARCIdentifiedPayloadType;
pub use self::warc_segment_number::WARCSegmentNumber;
pub use self::warc_segment_origin_id::WARCSegmentOriginID;
pub use self::warc_segment_total_length::WARCSegmentTotalLength;

mod warc_record_id;
mod warc_type;
mod warc_date;
mod content_length;
mod content_type;
mod warc_concurrent_to;
mod warc_block_digest;
mod warc_payload_digest;
mod warc_ip_address;
mod warc_refers_to;
mod warc_target_uri;
mod warc_truncated;
mod warc_warcinfo_id;
mod warc_filename;
mod warc_profile;
mod warc_identified_payload_type;
mod warc_segment_number;
mod warc_segment_origin_id;
mod warc_segment_total_length;
