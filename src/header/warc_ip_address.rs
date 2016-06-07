extern crate hyper;

header! {
    /// `WARC-IP-Address` header, defined in ISO28500; section 5.10
    ///
    /// The numeric Internet address contacted to retrieve any included content. An
    /// IPv4 address shall be written as a "dotted quad"; an IPv6 address shall be
    /// written as per [RFC1884]. For an HTTP retrieval, this will be the IP address
    /// used at retrieval time corresponding to the hostname in the record's
    /// target-URI.
    ///
    /// # ABNF
    /// ```plain
    /// WARC-IP-Address   = "WARC-IP-Address" ":" (ipv4 | ipv6)
    /// ipv4              = <"dotted quad">
    /// ipv6              = <per section 2.2 of RFC1884>
    /// ```
    ///
    /// The `WARC-IP-Address` field may be used on `response`, `resource`,
    /// `request`, `metadata`, and `revisit` records, but shall not be used on
    /// `warcinfo`, `conversion` or `continuation` records.
    (WARCIPAddress, "WARC-IP-Address") => [String]
}
