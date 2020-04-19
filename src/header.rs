use std::fmt;
use std::slice::Iter;
use std::vec::IntoIter;

#[derive(Clone, Debug, PartialEq)]
pub struct WarcHeader {
    pub token: String,
    pub value: Vec<u8>,
}

impl WarcHeader {
    pub fn new(token: &str, value: String) -> Self {
        WarcHeader {
            token: token.to_owned(),
            value: value.into_bytes(),
        }
    }
}

impl fmt::Display for WarcHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{}: {}",
            self.token,
            String::from_utf8_lossy(&self.value)
        )?;

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WarcHeaderRef<'a> {
    pub token: &'a str,
    pub value: &'a [u8],
}

impl<'a> fmt::Display for WarcHeaderRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{}: {}",
            self.token,
            String::from_utf8_lossy(&self.value)
        )?;

        Ok(())
    }
}

impl<'a> From<WarcHeaderRef<'a>> for WarcHeader {
    fn from(header_ref: WarcHeaderRef) -> Self {
        WarcHeader {
            token: header_ref.token.to_owned(),
            value: header_ref.value.to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WarcHeaders {
    inner: Vec<WarcHeader>,
}

impl<'a> From<WarcHeadersRef<'a>> for WarcHeaders {
    fn from(headers_ref: WarcHeadersRef) -> Self {
        WarcHeaders {
            inner: headers_ref
                .inner
                .into_iter()
                .map(|header_ref| header_ref.into())
                .collect(),
        }
    }
}

impl WarcHeaders {
    pub fn new(headers: Vec<WarcHeader>) -> Self {
        WarcHeaders { inner: headers }
    }

    pub fn with_capacity(size: usize) -> Self {
        WarcHeaders {
            inner: Vec::with_capacity(size),
        }
    }

    pub fn push(&mut self, header: WarcHeader) {
        self.inner.push(header);
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn iter(&self) -> Iter<'_, WarcHeader> {
        self.inner.iter()
    }
}

impl IntoIterator for WarcHeaders {
    type Item = WarcHeader;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl fmt::Display for WarcHeaders {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for header in self.inner.iter() {
            write!(f, "{}", header)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WarcHeadersRef<'a> {
    inner: Vec<WarcHeaderRef<'a>>,
}

impl<'a> WarcHeadersRef<'a> {
    pub fn new(headers: Vec<WarcHeaderRef<'a>>) -> Self {
        WarcHeadersRef { inner: headers }
    }

    pub fn with_capacity(size: usize) -> Self {
        WarcHeadersRef {
            inner: Vec::with_capacity(size),
        }
    }

    pub fn push(&mut self, header: WarcHeaderRef<'a>) {
        self.inner.push(header);
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn iter(&self) -> Iter<'_, WarcHeaderRef<'a>> {
        self.inner.iter()
    }
}

impl<'a> fmt::Display for WarcHeadersRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for header in self.inner.iter() {
            write!(f, "{}", header)?;
        }

        Ok(())
    }
}

pub const CONTENT_LENGTH: &str = "content-length";
pub const CONTENT_TYPE: &str = "content-type";
pub const WARC_BLOCK_DIGEST: &str = "warc-block-digest";
pub const WARC_CONCURRENT_TO: &str = "warc-concurrent-to";
pub const WARC_DATE: &str = "warc-date";
pub const WARC_FILENAME: &str = "warc-filename";
pub const WARC_IDENTIFIED_PAYLOAD_TYPE: &str = "warc-identified-payload-type";
pub const WARC_IP_ADDRESS: &str = "warc-ip-address";
pub const WARC_PAYLOAD_DIGEST: &str = "warc-payload-digest";
pub const WARC_PROFILE: &str = "warc-profile";
pub const WARC_RECORD_ID: &str = "warc-record-id";
pub const WARC_REFERS_TO: &str = "warc-refers-to";
pub const WARC_SEGMENT_NUMBER: &str = "warc-segment-number";
pub const WARC_SEGMENT_ORIGIN_ID: &str = "warc-segment-origin-id";
pub const WARC_SEGMENT_TOTAL_LENGTH: &str = "warc-segment-total-length";
pub const WARC_TARGET_URI: &str = "warc-target-uri";
pub const WARC_TRUNCATED: &str = "warc-truncated";
pub const WARC_TYPE: &str = "warc-type";
pub const WARC_WARCINFO_ID: &str = "warc-warcinfo-id";
