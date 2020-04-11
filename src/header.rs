pub type WarcHeaders<'a> = Vec<WarcHeader<'a>>;

#[derive(Clone, Debug, PartialEq)]
pub struct WarcHeader<'a> {
    pub token: &'a str,
    pub value: &'a [u8],
    pub delim_left: &'a [u8],
    pub delim_right: &'a [u8],
}

impl<'a> WarcHeader<'a> {
    pub fn new(token: &'a str, value: &'a [u8]) -> Self {
        WarcHeader {
            token: token,
            value: value,
            delim_left: "".as_bytes(),
            delim_right: " ".as_bytes(),
        }
    }
}

macro_rules! warc_headers {
    (
        $(
            $(#[$docs:meta])*
            ($upcase:ident, $name:expr);
        )+
    ) => {
        $(
            $(#[$docs])*
            pub const $upcase: &'static str = $name;
        )+
    }
}

// Generate constants for all warc headers.
warc_headers! {
    /// `Content-Length` header, defined in ISO28500; section 5.3
    ///
    /// The number of octets in the block, similar to [RFC2616]. If no block is
    /// present, a value of '0' (zero) shall be used.
    ///
    /// # ABNF
    /// ```plain
    /// Content-Length = "Content-Length" ":" 1*DIGIT
    /// ```
    ///
    /// All records shall have a Content-Length field.
    (CONTENT_LENGTH, "content-length");

    /// `Content-Type` header, defined in ISO28500; section 5.6
    ///
    /// The MIME type [RFC2045] of the information contained in the record's block.
    /// For example, in HTTP request and response records, this would be
    /// `application/http` as per section 19.1 of [RFC2616] (or `application/http;
    /// msgtype=request` and `application/http; msgtype=response` respectively). In
    /// particular, the content-type is not the value of the HTTP Content-Type
    /// header in an HTTP response but a MIME type to describe the full archived
    /// HTTP message (hence `application/http` if the block contains request or
    /// response headers).
    ///
    /// # ABNF
    /// ```plain
    /// Content-Type   = "Content-Type" ":" media-type
    /// media-type     = type "/" subtype *( ";" parameter )
    /// type           = token
    /// subtype        = token
    /// parameter      = attribute "=" value
    /// attribute      = token
    /// value          = token | quoted-string
    /// ```
    ///
    /// All records with a non-empty block (non-zero Content-Length), except
    /// `continuation` records, should have a Content-Type field. Only if the media
    /// type is not given by a Content-Type field, a reader may attempt to guess the
    /// media type via inspection of its content and/or the name extension(s) of the
    /// URI used to identify the resource. If the media type remains unknown, the
    /// reader should treat it as type `application/octet-stream`.
    (CONTENT_TYPE, "content-type");

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
    (WARC_BLOCK_DIGEST, "warc-block-digest");

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
    (WARC_CONCURRENT_TO, "warc-concurrent-to");

    /// `WARC-Date` header, defined in ISO28500; section 5.4
    ///
    /// A 14-digit UTC timestamp formatted according to YYYY-MM-DDThh:mm:ssZ,
    /// described in the W3C profile of ISO8601 [W3CDTF]. The timestamp shall
    /// represent the instant that data capture for record creation began. Multiple
    /// records written as part of a single capture event (see section 5.7) shall
    /// use the same `WARC-Date`, even though the times of their writing will not be
    /// exactly synchronized.
    ///
    /// # ABNF
    /// ```plain
    /// WARC-Date   = "WARC-Date" ":" w3c-iso8601
    /// w3c-iso8601 = <YYYY-MM-DDThh:mm:ssZ>
    /// ```
    ///
    /// All records shall have a `WARC-Date` field.
    (WARC_DATE, "warc-date");

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
    (WARC_FILENAME, "warc-filename");

    /// `WARC-Identified-Payload-Type` header, defined in ISO28500; section 5.17
    ///
    /// The `content-type` of the record's payload as determined by an independent
    /// check. This string shall not be arrived at by blindly promoting an HTTP
    /// `Content-Type` value up from a record block into the WARC header without
    /// direct analysis of the payload, as such values may often be unreliable.
    ///
    /// # ABNF
    /// ```plain
    ///  WARC-Identified-Payload-Type = "WARC-Identified-Payload-Type" ":" media-type
    /// ```
    ///
    /// The `WARC-Identified-Payload-Type` field may be used on WARC records with a
    /// well-defined payload and shall not be used on records without a well-defined
    /// payload.
    (WARC_IDENTIFIED_PAYLOAD_TYPE, "warc-identified-payload-type");

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
    (WARC_IP_ADDRESS, "warc-ip-address");

    /// `WARC-Payload-Digest` header, defined in ISO28500; section 5.9
    ///
    /// An optional parameter indicating the algorithm name and calculated value of
    /// a digest applied to the payload referred to or contained by the record -
    /// which is not necessarily equivalent to the record block.
    ///
    /// # ABNF
    /// ```plain
    /// WARC-Payload-Digest = "WARC-Payload-Digest" ":" labelled-digest
    /// ```
    ///
    /// An example is a SHA-1 labelled Base32 ([RFC3548]) value:
    ///
    /// ```plain
    /// WARC-Payload-Digest: sha1:3EF4GH5IJ6KL7MN8OPQAB2CD
    /// ```
    ///
    /// This document recommends no particular algorithm.
    ///
    /// The payload of an `application/http` block is its `entity-body` (per
    /// [RFC2616]). In contrast to `WARC-Block-Digest`, the `WARC-Payload-Digest`
    /// field may also be used for data not actually present in the current record
    /// block, for example when a block is left off in accordance with a `revisit`
    /// profile (see `revisit`), or when a record is segmented (the
    /// WARC-Payload-Digest recorded in the first segment of a segmented record
    /// shall be the digest of the payload of the logical record).
    ///
    /// The `WARC-Payload-Digest` field may be used on WARC records with a
    /// well-defined payload and shall not be used on records without a well-defined
    /// payload.
    (WARC_PAYLOAD_DIGEST, "warc-payload-digest");

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
    (WARC_PROFILE, "warc-profile");

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
    (WARC_RECORD_ID, "warc-record-id");

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
    (WARC_REFERS_TO, "warc-refers-to");

    /// `WARC-Segment-Number` header, defined in ISO28500; section 5.18
    ///
    /// Reports the current record's relative ordering in a sequence of segmented
    /// records.
    ///
    /// # ABNF
    /// ```plain
    /// WARC-Segment-Number = "WARC-Segment-Number" ":" 1*DIGIT
    /// ```
    ///
    /// In the first segment of any record that is completed in one or more later
    /// `continuation` WARC records, this parameter is mandatory. Its value there is
    /// `1`. In a `continuation` record, this parameter is also mandatory. Its value
    /// is the sequence number of the current segment in the logical whole record,
    /// increasing by 1 in each next segment.
    ///
    /// See the section below, Record Segmentation, for full details on the use of
    /// WARC record segmentation.
    (WARC_SEGMENT_NUMBER, "warc-segment-number");

    /// `WARC-Segment-Origin-ID` header, defined in ISO28500; section 5.18
    ///
    /// Identifies the starting record in a series of segmented records whose
    /// content blocks are reassembled to obtain a logically complete content block.
    ///
    /// # ABNF
    /// ```plain
    /// WARC-Segment-Origin-ID = "WARC-Segment-Origin-ID" ":" uri
    /// ```
    ///
    /// This field is mandatory on all `continuation` records, and shall not be used
    /// in other records. See the section below, Record segmentation, for full
    /// detail s on the use of WARC record segmentation.
    (WARC_SEGMENT_ORIGIN_ID, "warc-segment-origin-id");

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
    (WARC_SEGMENT_TOTAL_LENGTH, "warc-segment-total-length");

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
    (WARC_TARGET_URI, "warc-target-uri");

    /// `WARC-Truncated` header, defined in ISO28500; section 5.13
    ///
    /// For practical reasons, writers of the WARC format may place limits on the
    /// time or storage allocated to archiving a single resource. As a result, only
    /// a truncated portion of the original resource may be available for saving
    /// into a WARC record.
    ///
    /// Any record may indicate that truncation of its content block has occurred
    /// and give the reason with a `WARC-Truncated` field.
    ///
    /// # ABNF
    /// ```plain
    /// WARC-Truncated  = "WARC-Truncated" ":" reason-token
    /// reason-token    = "length"         ; exceeds configured max length
    ///                 | "time"           ; exceeds configured max time
    ///                 | "disconnect"     ; network disconnect
    ///                 | "unspecified"    ; other/unknown reason
    ///                 | future-reason
    /// future-reason   = token
    /// ```
    ///
    /// For example, if the capture of what appeared to be a multi-gigabyte resource
    /// was cut short after a transfer time limit was reached, the partial resource
    /// could be saved to a WARC record with this field.
    ///
    /// The `WARC-Truncated` field may be used on any WARC record. The WARC field
    /// `Content-Length` shall still report the actual truncated size of the record
    /// block.
    (WARC_TRUNCATED, "warc-truncated");

    /// `WARC-Type` header, defined in ISO28500; section 5.5
    ///
    /// The type of WARC record: one of `warcinfo`, `response`, `resource`,
    /// `request`, `metadata`, `revisit`, `conversion`, or `continuation`. Other
    /// types of WARC records may be defined in extensions of the core format. Types
    /// are further described in WARC Record Types.
    ///
    /// A WARC file needs not contain any particular record types, though starting
    /// all WARC files with a `warcinfo` record is recommended.
    ///
    /// # ABNF
    /// ```plain
    /// WARC-Type   = "WARC-Type" ":" record-type
    /// record-type = "warcinfo" | "response" | "resource"
    ///             | "request" | "metadata" | "revisit"
    ///             | "conversion" | "continuation" |  future-type
    /// future-type = token
    /// ```
    ///
    /// All records shall have a `WARC-Type` field.
    ///
    /// WARC processing software shall ignore records of unrecognized type.
    (WARC_TYPE, "warc-type");

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
    (WARC_WARCINFO_ID, "warc-warcinfo-id");
}
