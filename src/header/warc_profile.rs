extern crate hyper;

header! {
    /// `WARC-Profile` header, defined in ISO28500; section 5.16
    ///
    /// A URI signifying the kind of analysis and handling applied in a `revisit`
    /// record. (Like an XML namespace, the URI may, but need not, return
    /// human-readable or machine-readable documentation.) If reading software does
    /// not recognize the given URI as a supported kind of handling, it shall not
    /// attempt to interpret the associated record block.
    ///
    /// # ABNF
    /// ```plain
    /// WARC-Profile = "WARC-Profile" ":" uri
    /// ```
    ///
    /// The section `revisit` defines two initial profile options for the
    /// WARC-Profile header for `revisit` records.
    ///
    /// The `WARC-Profile` field is mandatory on `revisit` type records and
    /// undefined for other record types.
    (WarcProfile, "WARC-Profile") => [String]
}
