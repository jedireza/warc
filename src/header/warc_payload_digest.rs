/// `WARC-Payload-Digest` header, defined in ISO28500; section 5.9
///
/// An optional parameter indicating the algorithm name and calculated value of
/// a digest applied to the payload referred to or contained by the record -
/// which is not necessarily equivalent to the record block. 
///
/// # ABNF
/// ```plain
/// WARC-Payload-Digest = "WARC-Payload-Digest" ":" labelled-digest 
/// ```
///
/// An example is a SHA-1 labelled Base32 ([RFC3548]) value:
///
/// ```plain
/// WARC-Payload-Digest: sha1:3EF4GH5IJ6KL7MN8OPQAB2CD 
/// ```
///
/// This document recommends no particular algorithm. 
///
/// The payload of an `application/http` block is its `entity-body` (per
/// [RFC2616]). In contrast to `WARC-Block-Digest`, the `WARC-Payload-Digest`
/// field may also be used for data not actually present in the current record
/// block, for example when a block is left off in accordance with a `revisit`
/// profile (see `revisit`), or when a record is segmented (the
/// WARC-Payload-Digest recorded in the first segment of a segmented record
/// shall be the digest of the payload of the logical record). 
///
/// The `WARC-Payload-Digest` field may be used on WARC records with a
/// well-defined payload and shall not be used on records without a well-defined
/// payload. 
#[derive(Clone, Debug, PartialEq)]
pub struct WARCPayloadDigest(pub String);
