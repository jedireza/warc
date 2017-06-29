extern crate hyper;

header! {
    /// `WARC-Concurrent-To` header, defined in ISO28500; section 5.7
    ///
    /// The `WARC-Record-ID`s of any records created as part of the same capture
    /// event as the current record. A capture event comprises the information
    /// automatically gathered by a retrieval against a single target-URI; for
    /// example, it might be represented by a `response` or `revisit` record plus
    /// its associated `request` record.
    ///
    /// # ABNF
    /// ```plain
    /// WARC-Concurrent-To = "WARC-Concurrent-To" ":" uri
    /// ```
    ///
    /// This field may be used to associate records of types `request`, `response`,
    /// `resource`, `metadata`, and `revisit` with one another when they arise from
    /// a single capture event (When so used, any `WARC-Concurrent-To` association
    /// shall be considered bidirectional even if the header only appears on one
    /// record.) The `WARC-Concurrent-To` field shall not be used in `warcinfo`,
    /// `conversion`, and `continuation` records.
    ///
    /// As an exception to the general rule, it is allowed to repeat several
    /// `WARC-Concurrent-To` fields within the same WARC record.
    (WarcConcurrentTo, "WARC-Concurrent-To") => [String]
}
