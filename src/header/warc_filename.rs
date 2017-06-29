extern crate hyper;

header! {
    /// `WARC-Filename` header, defined in ISO28500; section 5.15
    ///
    /// The filename containing the current `warcinfo` record
    ///
    /// # ABNF
    /// ```plain
    /// WARC-Filename = "WARC-Filename" ":" ( TEXT | quoted-string )
    /// ```
    ///
    /// The `WARC-Filename` field may be used in `warcinfo` type records and shall
    /// not be used for other record types.
    (WarcFilename, "WARC-Filename") => [String]
}
