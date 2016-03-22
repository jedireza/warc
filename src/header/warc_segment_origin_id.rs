/// `WARC-Segment-Origin-ID` header, defined in ISO28500; section 5.18
///
/// Identifies the starting record in a series of segmented records whose
/// content blocks are reassembled to obtain a logically complete content block. 
///
/// # ABNF
/// ```plain
/// WARC-Segment-Origin-ID = "WARC-Segment-Origin-ID" ":" uri
/// ```
///
/// This field is mandatory on all `continuation` records, and shall not be used
/// in other records. See the section below, Record segmentation, for full
/// detail s on the use of WARC record segmentation.  
#[derive(Clone, Debug, PartialEq)]
pub struct WARCSegmentOriginID(pub String);
