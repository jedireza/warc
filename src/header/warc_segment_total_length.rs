extern crate hyper;

header! {
    /// `WARC-Segment-Total-Length` header, defined in ISO28500; section 5.18
    ///
    /// In the final record of a segmented series, reports the total length of all
    /// segment content blocks when concatenated together.
    ///
    /// # ABNF
    /// ```plain
    /// WARC-Segment-Total-Length = "WARC-Segment-Total-Length" ":" 1*DIGIT
    /// ```
    ///
    /// This field is mandatory on the last `continuation` record of a series, and
    /// shall not be used elsewhere.
    ///
    /// See the section below, Record segmentation, for full details on the use of
    /// WARC record segmentation.
    (WARCSegmentTotalLength, "WARC-Segment-Total-Length") => [u64]
}
