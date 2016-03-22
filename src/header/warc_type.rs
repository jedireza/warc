extern crate hyper;

use hyper::header::{Header, HeaderFormat};
use hyper::header::parsing::from_one_raw_str;
use std::fmt;
use std::str;
use std::str::FromStr;

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
#[derive(Clone, Debug, PartialEq)]
pub struct WARCType(pub WARCRecordType);

impl Header for WARCType {
    fn header_name() -> &'static str {
        "WARC-Type"
    }

    fn parse_header(raw: &[Vec<u8>]) -> hyper::error::Result<WARCType> {
        from_one_raw_str(raw).and_then(|val: String| {
            WARCType::from_str(&val)
        })
    }
}

impl HeaderFormat for WARCType {
    fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for WARCType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_header(f)
    }
}

impl str::FromStr for WARCType {
    type Err = hyper::error::Error;

    fn from_str(val: &str) -> hyper::error::Result<WARCType> {
        match val {
            "warcinfo" => Ok(WARCType(WARCRecordType::WARCInfo)),
            "response" => Ok(WARCType(WARCRecordType::Response)),
            "resource" => Ok(WARCType(WARCRecordType::Resource)),
            "request" => Ok(WARCType(WARCRecordType::Request)),
            "metadata" => Ok(WARCType(WARCRecordType::Metadata)),
            "revisit" => Ok(WARCType(WARCRecordType::Revisit)),
            "conversion" => Ok(WARCType(WARCRecordType::Conversion)),
            "continuation" => Ok(WARCType(WARCRecordType::Continuation)),
            _ => Ok(WARCType(WARCRecordType::Unknown(val.to_owned()))),
        }
    }
}

/// Variants for `WARCType` header
///
/// As defined in ISO28500; section 6.1
///
/// The purpose and use of each defined record type is described below.
///
/// Because new record types that extend the WARC format may be defined in
/// future standards, WARC processing software shall skip records of unknown
/// type.
#[derive(Clone, Debug, PartialEq)]
pub enum WARCRecordType {
    /// `warcinfo` as defined in ISO28500; section 6.2
    ///
    /// A `warcinfo` record describes the records that follow it, up through end
    /// of file, end of input, or until next `warcinfo` record. Typically, this
    /// appears once and at the beginning of a WARC file. For a web archive, it
    /// often contains information about the web crawl which generated the
    /// following records.
    ///
    /// The format of this descriptive record block may vary, though the use of
    /// the `application/warc-fields` `content-type` is recommended. Allowable
    /// fields include, but are not limited to, all [DCMI] plus the following
    /// field definitions. All fields are optional
    ///
    /// > `operator` - Contact information for the operator who created this
    /// WARC resource. A name or name and email address is recommended.
    ///
    /// > `software` - The software and software version used to create this
    /// WARC resource. For example, `heritrix/1.12.0`.
    ///
    /// > `robots` - The robots policy followed by the harvester creating this
    /// WARC resource. The string `classic` indicates the 1994 web robots
    /// exclusion standard rules are being obeyed.
    ///
    /// > `hostname` - The hostname of the machine that created this WARC
    /// resource, such as `crawling17.archive.org`.
    ///
    /// > `ip` - The IP address of the machine that created this WARC resource,
    /// such as `123.2.3.4`.
    ///
    /// > `http-header-user-agent` - The HTTP `user-agent` header usually sent
    /// by the harvester along with each request. Note that if `request` records
    /// are used to save verbatim requests, this information is redundant. (If a
    /// `request` or `metadata` record reports a different `user-agent` for a
    /// specific request, the more specific information should be considered
    /// more reliable.)
    ///
    /// > `http-header-from` - The HTTP `From` header usually sent by the
    /// harvester along with each request. (The same considerations as for
    /// `user-agent` apply.)
    ///
    /// So that multiple record excerpts from inside WARC files are also valid
    /// WARC files, it is optional that the first record of a legal WARC be a
    /// `warcinfo` description. Also, to allow the concatenation of WARC files
    /// into a larger valid WARC file, it is allowable for `warcinfo` records to
    /// appear in the middle of a WARC file.
    ///
    /// See annex C.1 below for an example of a `warcinfo` record.
    WARCInfo,
    /// `response` as defined in ISO28500; section 6.3
    ///
    /// ### General
    ///
    /// A `response` record should contain a complete scheme-specific response,
    /// including network protocol information where possible. The exact
    /// contents of a `response` record are determined not just by the record
    /// type but also by the URI scheme of the record's target-URI, as described
    /// below.
    ///
    /// See annex C.2 below for an example of a ‘response’ record.
    ///
    /// ### for `http` and `https` scheme
    ///
    /// For a target-URI of the `http` or `https` schemes, a `response` record
    /// block should contain the full HTTP response received over the network,
    /// including headers. That is, it contains the `response` message defined
    /// by section 6 of HTTP/1.1 (RFC2616), or by any previous or subsequent
    /// version of HTTP compatible with the section 6 of HTTP/1.1 (RFC2616).
    ///
    /// The WARC record's Content-Type field should contain the value defined by
    /// HTTP/1.1, `application/http;msgtype=response`. When software bugs,
    /// network issues, or implementation limits cause response-like material to
    /// be collected that is not perfectly compliant with HTTP specifications,
    /// WARC writing software may record the problematic content us ing its best
    /// effort determination of the interesting material boundaries. That is,
    /// neither the use of the `response` record with an `http` target-URI nor
    /// the `application/http` `content-type` serves as an absolute guarantee
    /// that the contained material is a legal HTTP response.
    ///
    /// A `WARC-IP-Address` field should be used to record the network IP
    /// address from which the response material was received.
    ///
    /// When a `response` is known to have been truncated, this shall be noted
    /// using the `WARC-Truncated` field.
    ///
    /// A `WARC-Concurrent-To` field (or fields) may be used to associate
    /// the `response` to a matching `request` record or concurrently-created
    /// `metadata` record.
    ///
    /// The payload of a `response` record with a target-URI of scheme `http` or
    /// `https` is defined as its `entity-body` (per [RFC2616]), with any
    /// transfer-encoding removed. If a truncated `response` record block
    /// contains less than the full entity-body, the payload is considered
    /// truncated at the same position.
    ///
    /// This document does not specify conventions for recording information
    /// about the `https` secure socket transaction, such as certificates
    /// exchanged, consulted, or verified.
    ///
    /// ### for other URI schemes
    ///
    /// This document does not specify the contents of the `response` record for
    /// other URI schemes.
    Response,
    /// `resource` as defined in ISO28500; section 6.4
    ///
    /// ### General
    ///
    /// A `resource` record contains a resource, without full protocol response
    /// information. For example: a file directly retrieved from a locally
    /// accessible repository or the result of a networked retrieval where the
    /// protocol information has been discarded. The exact contents of a
    /// `resource` record are determined not just by the record type but also by
    /// the URI scheme of the record's `target-uri`, as described below.
    ///
    /// For all `resource` records, the payload is defined as the record block.
    ///
    /// A `resource` record, with a synthesized `target-uri`, may also be used
    /// to archive other artefacts of a harvesting process inside WARC files.
    ///
    /// See annex C.3 below for an example of a ‘resource’ record.
    ///
    /// ### for `http` and `https` schemes
    ///
    /// For a target-URI of the `http` or `https` schemes, a `resource` record
    /// block shall contain the returned `entity-body` (per [RFC2616], with any
    /// transfer-encodings removed), possibly truncated.
    ///
    /// ### for `ftp` scheme
    ///
    /// For a target-URI of the `ftp` scheme, a `resource` record block shall
    /// contain the complete file returned by an FTP operation, possibly
    /// truncated.
    ///
    /// ### for `dns` scheme
    ///
    /// For a `target-uri` of the `dns` scheme ([RFC4501]), a `resource` record
    /// shall contain material of `content-type` 'text/dns' (registered by
    /// [RFC4027] and defined by [RFC2540] and [RFC1035]) representing the
    /// results of a single DNS lookup as described by the `target-uri`.
    ///
    /// ### for other URI schemes
    ///
    /// This document does not specify the contents of the `resource` record for
    /// other URI schemes
    Resource,
    /// `request` as defined in ISO28500; section 6.5
    ///
    /// ### General
    ///
    /// A `request` record holds the details of a complete scheme-specific
    /// request, including network protocol information where possible. The
    /// exact contents of a `request` record are determined not just by the
    /// record type but also by the URI scheme of the record's `target-uri`, as
    /// described below.
    ///
    /// See annex C.4 below for an example of a `request` record.
    ///
    /// ### for `http` and `https` schemes
    ///
    /// For a `target-uri` of the `http` or `https` schemes, a `request` record
    /// block should contain the full HTTP request sent over the network,
    /// including headers. That is, it contains the `request` message defined by
    /// section 5 of HTTP/1.1 (RFC2616), or by any previous or subsequent
    /// version of HTTP compatible with the section 5 of HTTP/1.1 (RFC2616).
    ///
    /// The WARC record's Content-Type field should contain the value defined by
    /// HTTP/1.1, `application/http;msgtype=request`.
    ///
    /// A `WARC-IP-Address` field should be used to record the network IP address
    /// to which the request material was directed.
    ///
    /// A `WARC-Concurrent-To` field (or fields) may be used to associate the
    /// `request` to a matching `response` record or concurrently-created
    /// `metadata` record.
    ///
    /// The payload of a `request` record with a `target-uri` of scheme `http`
    /// or `https` is defined as its `entity-body` (per [RFC2616]), with any
    /// transfer-encoding removed. If a truncated `request` record block
    /// contains less than the full entity-body, the payload is considered
    /// truncated at the same position.
    ///
    /// This document does not specify conventions for recording information
    /// about the `https` secure socket transaction, such as certificates
    /// exchanged, consulted, or verified.
    ///
    /// ### for other URI schemes
    ///
    /// This document does not specify the contents of the `request` record for
    /// other URI schemes.
    Request,
    /// `metadata` as defined in ISO28500; section 6.6
    ///
    /// A `metadata` record contains content created in order to further
    /// describe, explain, or accompany a harvested resource, in ways not
    /// covered by other record types.  A `metadata` record will almost always
    /// refer to another record of another type, with that other record holding
    /// original harvested or transformed content. (However, it is allowable for
    /// a `metadata` record to refer to any record type, including other
    /// `metadata` records.) Any number of metadata records may reference one
    /// specific other record.
    ///
    /// The format of the metadata record block may vary. The
    /// `application/warc-fields` format, defined earlier, may be used.
    /// Allowable fields include all [DCMI] plus the following field
    /// definitions. All fields are optional.
    ///
    /// > `via` - The referring URI from which the archived URI was discovered.
    ///
    /// > `hopsFromSeed` - A symbolic string describing the type of each hop
    /// from a starting `seed` URI to the current URI.
    ///
    /// > `fetchTimeMs` - Time in milliseconds that it took to collect the
    /// archived URI, starting from the initiation of network traffic.
    ///
    /// A `metadata` record may be associated with other records derived from
    /// the same capture event using the `WARC-Concurrent-To` header. A `metadata`
    /// record may be associated to another record which it describes using the
    /// `WARC-Refers-To` header.
    ///
    /// See annex C.5 below for an example of a ‘metadata’ record.
    Metadata,
    /// `revisit` as defined in ISO28500; section 6.7
    ///
    /// ### General
    ///
    /// A `revisit` record describes the revisitation of content already
    /// archived, and might include only an abbreviated content body which has
    /// to be interpreted relative to a previous record. Most typically, a
    /// `revisit` record is used instead of a `response` or `resource` record to
    /// indicate that the content visited was either a complete or substantial
    /// duplicate of material previously archived.
    ///
    /// Using a `revisit` record instead of another type is optional, for when
    /// benefits of reduced storage size or improved cross-referencing of
    /// material are desired.
    ///
    /// A `revisit` record shall contain a `WARC-Profile` field which determines
    /// the interpretation of the record's fields and record block. Two initial
    /// values and their interpretation are described in the following sections.
    /// A reader which does not recognize the profile URI shall not attempt to
    /// interpret the enclosing record or associated content body.
    ///
    /// The purpose of this record type is to reduce storage redundancy when
    /// repeatedly retrieving identical or little-changed content, while still
    /// recording that a revisit occurred, plus details about the current state
    /// of the visited content relative to the archived version.
    ///
    /// See annex C.6 below for an example of a ‘revisit’ record.
    ///
    /// ### Profile: Identical Payload Digest
    ///
    /// This `revisit` profile may be used whenever a subsequent consideration
    /// of a URI provides payload content which a strong digest function, such
    /// as SHA-1, indicates is identical to a previously recorded version.
    ///
    /// To indicate this profile, use the URI:
    ///
    /// ```plain
    /// http://netpreserve.org/warc/1.0/revisit/identical-payload-digest
    /// ```
    ///
    /// To report the payload digest used for comparison, a `revisit` record
    /// using this profile shall include a `WARC-Payload-Digest` field, with a
    /// value of the digest that was calculated on the payload.
    ///
    /// A `revisit` record using this profile may have no record block, in which
    /// case a Content-Length of zero must be written. If a record block is
    /// present, it shall be interpreted the same as a `response` record type
    /// for the same URI, but truncated to avoid storing the duplicate content.
    /// A `WARC-Truncated` header with reason `length` shall be used for any
    /// identical-digest truncation.
    ///
    /// For records using this profile, the payload is defined as the original
    /// payload content whose digest value was unchanged.
    ///
    /// Using a `WARC-Refers-To` header to identify a specific prior record from
    /// which the matching content can be retrieved is recommended, to minimize
    /// the risk of misinterpreting the `revisit` record.
    ///
    /// ### Profile: Server Not Modified
    ///
    /// This `revisit` profile may be used whenever a subsequent consideration
    /// of a URI encounters an assertion from the providing server that the
    /// content has not changed, such as an HTTP "304 Not Modified" response.
    ///
    /// To indicate this profile, use the URI:
    ///
    /// ```plain
    /// http://netpreserve.org/warc/1.0/revisit/server-not-modified
    /// ```
    ///
    /// A `revisit` record using this profile may have no content body, in which
    /// case a Content-Length of zero shall be written. If a content body is
    /// present, it should be interpreted the same as a `response` record type
    /// for the same URI, truncated if desired.
    ///
    /// For records using this profile, the payload is defined as the original
    /// payload content from which a `Last-Modified` and/or `ETag` value was
    /// taken.
    ///
    /// Using a `WARC-Refers-To` header to identify a specific prior record from
    /// which the unmodified content can be retrieved is recommended, to
    /// minimize the risk of misinterpreting the `revisit` record.
    ///
    /// ### Other profiles
    ///
    /// Other documents may define additional profiles to accomplish other
    /// goals, such as recording the apparent magnitude of difference from the
    /// previous visit, or to encode the visited content as a "diff" -- where
    /// "diff" is the file comparison utility that outputs the differences
    /// between two files -- of the content previously stored.
    Revisit,
    /// `conversion` as defined in ISO28500; section 6.8
    ///
    /// A `conversion` record shall contain an alternative version of another
    /// record's content that was created as the result of an archival process.
    /// Typically, this is used to hold content transformations that maintain
    /// viability of content after widely available rendering tools for the
    /// originally stored format disappear. As needed, the original content may
    /// be migrated (transformed) to a more viable format in order to keep the
    /// information usable with current tools while minimizing loss of
    /// information (intellectual content, look and feel, etc). Any number of
    /// `conversion` records may be created that reference a specific source
    /// record, which may itself contain transformed content. Each
    /// transformation should result in a freestanding, complete record, with no
    /// dependency on survival of the original record.
    ///
    /// Metadata records may be used to further describe transformation records.
    /// Wherever practical, a `conversion` record should contain a
    /// `WARC-Refers-To` field to identify the prior material converted.
    ///
    /// For `conversion` records, the payload is defined as the record block.
    ///
    /// See annex C.7 below for an example of a ‘conversion’ record.
    Conversion,
    /// `continuation` as defined in ISO28500; section 6.9
    ///
    /// Record blocks from `continuation` records must be appended to
    /// corresponding prior record block(s) (e.g., from other WARC files) to
    /// create the logically complete full-sized original record. That is,
    /// `continuation` records are used when a record that would otherwise cause
    /// a WARC file size to exceed a desired limit is broken into segments. A
    /// continuation record shall contain the named fields
    /// `WARC-Segment-Origin-ID` and `WARC-Segment-Number`, and the last
    /// `continuation` record of a series shall contain a
    /// `WARC-Segment-Total-Length` field. The full details of WARC record
    /// segmentation are described in the below section Record Segmentation. See
    /// also annex C.8 below for an example of a `continuation` record.
    Continuation,
    /// for any other non-standard `WARC-Type`
    Unknown(String),
}

impl fmt::Display for WARCRecordType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            WARCRecordType::WARCInfo => "warcinfo",
            WARCRecordType::Response => "response",
            WARCRecordType::Resource => "resource",
            WARCRecordType::Request => "request",
            WARCRecordType::Metadata => "metadata",
            WARCRecordType::Revisit => "revisit",
            WARCRecordType::Conversion => "conversion",
            WARCRecordType::Continuation => "continuation",
            WARCRecordType::Unknown(ref val) => val.as_ref(),
        })
    }
}
