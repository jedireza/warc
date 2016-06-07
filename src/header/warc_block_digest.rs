extern crate hyper;

header! {
    /// `WARC-Block-Digest` header, defined in ISO28500; section 5.8
    ///
    /// An optional parameter indicating the algorithm name and calculated value of
    /// a digest applied to the full block of the record.
    ///
    /// # ABNF
    /// ```plain
    /// WARC-Block-Digest = "WARC-Block-Digest" ":" labelled-digest
    /// labelled-digest   = algorithm ":" digest-value
    /// algorithm         = token
    /// digest-value      = token
    /// ```
    ///
    /// An example is a SHA-1 labelled Base32 ([RFC3548]) value:
    ///
    /// ```plain
    /// WARC-Block-Digest: sha1:AB2CD3EF4GH5IJ6KL7MN8OPQ
    /// ```
    ///
    /// This document recommends no particular algorithm.
    ///
    /// Any record may have a `WARC-Block-Digest` field.
    (WARCBlockDigest, "WARC-Block-Digest") => [String]
}
