extern crate hyper;

header! {
    /// `WARC-Refers-To` header, defined in ISO28500; section 5.11
    ///
    /// The `WARC-Record-ID` of a single record for which the present record holds
    /// additional content.
    ///
    /// # ABNF
    /// ```plain
    /// WARC-Refers-To = "WARC-Refers-To" ":" uri
    /// ```
    ///
    /// The `WARC-Refers-To` field may be used to associate a `metadata` record to
    /// another record it describes. The `WARC-Refers-To` field may also be used to
    /// associate a record of type `revisit` or `conversion` with the preceding
    /// record which helped determine the present record content. The
    /// `WARC-Refers-To` field shall not be used in `warcinfo`, `response`,
    /// `resource`, `request`, and `continuation` records.
    (WarcRefersTo, "WARC-Refers-To") => [String]
}
