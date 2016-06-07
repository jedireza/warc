extern crate hyper;

header! {
    /// `WARC-Target-URI` header, defined in ISO28500; section 5.12
    ///
    /// The original URI whose capture gave rise to the information content in this
    /// record. In the context of web harvesting, this is the URI that was the
    /// target of a crawler's retrieval request. For a `revisit` record, it is the
    /// URI that was the target of a retrieval request. Indirectly, such as for a
    /// `metadata`, or `conversion` record, it is a copy of the `WARC-Target-URI`
    /// appearing in the original record to which the newer record pertains. The URI
    /// in this value shall be properly escaped according to [RFC3986] and written
    /// with no internal whitespace.
    ///
    /// # ABNF
    /// ```plain
    /// WARC-Target-URI = "WARC-Target-URI" ":" uri
    /// ```
    ///
    /// All `response`, `resource`, `request`, `revisit`, `conversion` and
    /// `continuation` records shall have a `WARC-Target-URI` field. A `metadata`
    /// record may have a `WARC-Target-URI` field. A `warcinfo` record shall not
    /// have a `WARC-Target-URI` field.
    (WARCTargetURI, "WARC-Target-URI") => [String]
}
