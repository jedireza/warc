extern crate hyper;

header! {
    /// `WARC-Record-ID` header, defined in ISO28500; section 5.2
    ///
    /// An identifier assigned to the current record that is globally unique for its
    /// period of intended use. No identifier scheme is mandated by this
    /// specification, but each record-id shall be a legal URI and clearly indicate
    /// a documented and registered scheme to which it conforms (e.g., via a URI
    /// scheme prefix such as "http:" or "urn:"). Care should be taken to ensure
    /// that this value is written with no internal whitespace.
    ///
    /// # ABNF
    /// ```plain
    /// WARC-Record-ID = "WARC-Record-ID" ":" uri
    /// ```
    ///
    /// All records shall have a `WARC-Record-ID` field.
    (WARCRecordID, "WARC-Record-ID") => [String]
}
