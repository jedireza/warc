/// `WARC-Identified-Payload-Type` header, defined in ISO28500; section 5.17
///
/// The `content-type` of the record's payload as determined by an independent
/// check. This string shall not be arrived at by blindly promoting an HTTP
/// `Content-Type` value up from a record block into the WARC header without
/// direct analysis of the payload, as such values may often be unreliable.
///
/// # ABNF
/// ```plain
///  WARC-Identified-Payload-Type = "WARC-Identified-Payload-Type" ":" media-type
/// ```
///
/// The `WARC-Identified-Payload-Type` field may be used on WARC records with a
/// well-defined payload and shall not be used on records without a well-defined
/// payload.
#[derive(Clone, Debug, PartialEq)]
pub struct WARCIdentifiedPayloadType(pub String);
