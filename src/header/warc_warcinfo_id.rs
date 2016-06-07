extern crate hyper;

header! {
    /// `WARC-Warcinfo-ID` header, defined in ISO28500; section 5.14
    ///
    /// When present, indicates the `WARC-Record-ID` of the associated `warcinfo`
    /// record for this record. Typically, the Warcinfo-ID parameter is used when
    /// the context of the applicable `warcinfo` record is unavailable, such as
    /// after distributing single records into separate WARC files. WARC writing
    /// applications (such web crawlers) may choose to always record this parameter.
    ///
    /// # ABNF
    /// ```plain
    /// WARC-Warcinfo-ID = "WARC-Warcinfo-ID" ":" uri
    /// ```
    ///
    /// The `WARC-Warcinfo-ID` field value overrides any association with a
    /// previously occurring (in the WARC) `warcinfo` record, thus providing a way
    /// to protect the true association when records are combined from different
    /// WARCs.
    ///
    /// The `WARC-Warcinfo-ID` field may be used in any record type except
    /// `warcinfo`.
    (WARCWarcinfoID, "WARC-Warcinfo-ID") => [String]
}
