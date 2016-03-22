/// `WARC-Truncated` header, defined in ISO28500; section 5.13
///
/// For practical reasons, writers of the WARC format may place limits on the
/// time or storage allocated to archiving a single resource. As a result, only
/// a truncated portion of the original resource may be available for saving
/// into a WARC record.
///
/// Any record may indicate that truncation of its content block has occurred
/// and give the reason with a `WARC-Truncated` field.
///
/// # ABNF
/// ```plain
/// WARC-Truncated  = "WARC-Truncated" ":" reason-token
/// reason-token    = "length"         ; exceeds configured max length
///                 | "time"           ; exceeds configured max time
///                 | "disconnect"     ; network disconnect
///                 | "unspecified"    ; other/unknown reason
///                 | future-reason
/// future-reason   = token
/// ```
///
/// For example, if the capture of what appeared to be a multi-gigabyte resource
/// was cut short after a transfer time limit was reached, the partial resource
/// could be saved to a WARC record with this field.
///
/// The `WARC-Truncated` field may be used on any WARC record. The WARC field
/// `Content-Length` shall still report the actual truncated size of the record
/// block.
#[derive(Clone, Debug, PartialEq)]
pub struct WARCTruncated(pub String);
