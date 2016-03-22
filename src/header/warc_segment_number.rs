/// `WARC-Segment-Number` header, defined in ISO28500; section 5.18
///
/// Reports the current record's relative ordering in a sequence of segmented
/// records. 
///
/// # ABNF
/// ```plain
/// WARC-Segment-Number = "WARC-Segment-Number" ":" 1*DIGIT
/// ```
///
/// In the first segment of any record that is completed in one or more later
/// `continuation` WARC records, this parameter is mandatory. Its value there is
/// `1`. In a `continuation` record, this parameter is also mandatory. Its value
/// is the sequence number of the current segment in the logical whole record,
/// increasing by 1 in each next segment. 
///
/// See the section below, Record Segmentation, for full details on the use of
/// WARC record segmentation.
#[derive(Clone, Debug, PartialEq)]
pub struct WARCSegmentNumber(pub String);
