use nom::{
    bytes::streaming::{tag, take, take_while1},
    character::streaming::{line_ending, not_line_ending, space0},
    error::ErrorKind,
    multi::many1,
    sequence::{delimited, tuple},
    IResult,
};
use std::str;

#[derive(Clone, Debug, PartialEq)]
pub struct WarcHeaderParsed<'a> {
    pub token: &'a str,
    pub value: &'a [u8],
    pub delim_left: &'a [u8],
    pub delim_right: &'a [u8],
}

#[derive(Clone, Debug, PartialEq)]
pub struct WarcRecordParsed<'a> {
    pub version: &'a [u8],
    pub headers: WarcHeadersParsed<'a>,
    pub body: &'a [u8],
}

pub type WarcHeadersParsed<'a> = Vec<WarcHeaderParsed<'a>>;

fn version(input: &[u8]) -> IResult<&[u8], &[u8]> {
    delimited(tag("WARC/"), not_line_ending, line_ending)(input)
}

fn is_header_token_char(chr: u8) -> bool {
    match chr {
        0..=31
        | 128..=255
        | b'('
        | b')'
        | b'<'
        | b'>'
        | b'@'
        | b','
        | b';'
        | b':'
        | b'"'
        | b'/'
        | b'['
        | b']'
        | b'?'
        | b'='
        | b'{'
        | b'}'
        | b' '
        | b'\\' => false,
        _ => true,
    }
}

fn header(input: &[u8]) -> IResult<&[u8], (&[u8], &[u8], &[u8], &[u8])> {
    let (input, (token, delim_left, _, delim_right, value, _)) = tuple((
        take_while1(is_header_token_char),
        space0,
        tag(":"),
        space0,
        not_line_ending,
        line_ending,
    ))(input)?;

    Ok((input, (token, value, delim_left, delim_right)))
}

fn headers(input: &[u8]) -> IResult<&[u8], (WarcHeadersParsed, usize)> {
    let (input, headers) = many1(header)(input)?;

    let mut content_length: Option<usize> = None;
    let mut warc_headers: WarcHeadersParsed = Vec::with_capacity(headers.len());

    for header in headers {
        let token_str = match str::from_utf8(header.0) {
            Err(_) => {
                return Err(nom::Err::Error((input, ErrorKind::Verify)));
            }
            Ok(token) => token,
        };

        if content_length == None && token_str.to_lowercase() == "content-length" {
            let value_str = match str::from_utf8(header.1) {
                Err(_) => {
                    return Err(nom::Err::Error((input, ErrorKind::Verify)));
                }
                Ok(value) => value,
            };

            match value_str.parse::<usize>() {
                Err(_) => {
                    return Err(nom::Err::Error((input, ErrorKind::Verify)));
                }
                Ok(len) => {
                    content_length = Some(len);
                }
            }
        }

        warc_headers.push(WarcHeaderParsed {
            token: token_str,
            value: header.1,
            delim_left: header.2,
            delim_right: header.3,
        });
    }

    if content_length == None {
        content_length = Some(0);
    }

    Ok((input, (warc_headers, content_length.unwrap())))
}

pub fn record(input: &[u8]) -> IResult<&[u8], WarcRecordParsed> {
    let (input, (version, headers, _)) = tuple((version, headers, line_ending))(input)?;
    let (input, (body, _, _)) = tuple((take(headers.1), line_ending, line_ending))(input)?;

    let record = WarcRecordParsed {
        version: version,
        headers: headers.0,
        body: body,
    };

    Ok((input, record))
}

#[cfg(test)]
mod tests {
    use super::{header, headers, record, version};
    use super::{WarcHeaderParsed, WarcHeadersParsed, WarcRecordParsed};
    use nom::error::ErrorKind;
    use nom::Err;
    use nom::Needed;

    #[test]
    fn version_parsing() {
        assert_eq!(version(&b"WARC/0.0\r\n"[..]), Ok((&b""[..], &b"0.0"[..])));

        assert_eq!(version(&b"WARC/1.0\r\n"[..]), Ok((&b""[..], &b"1.0"[..])));

        assert_eq!(
            version(&b"WARC/2.0-alpha\r\n"[..]),
            Ok((&b""[..], &b"2.0-alpha"[..]))
        );
    }

    #[test]
    fn header_pair_parsing() {
        assert_eq!(
            header(&b"some-header: all/the/things\r\n"[..]),
            Ok((
                &b""[..],
                (
                    &b"some-header"[..],
                    &b"all/the/things"[..],
                    &b""[..],
                    &b" "[..]
                )
            ))
        );

        assert_eq!(
            header(&b"another-header : with extra spaces\r\n"[..]),
            Ok((
                &b""[..],
                (
                    &b"another-header"[..],
                    &b"with extra spaces"[..],
                    &b" "[..],
                    &b" "[..]
                )
            ))
        );

        assert_eq!(
            header(&b"incomplete-header : missing-line-ending"[..]),
            Err(Err::Incomplete(Needed::Unknown))
        );
    }

    #[test]
    fn headers_parsing() {
        let raw_invalid = b"\
            content-length: R2D2\r\n\
            that: is not\r\n\
            a-valid: content-length\r\n\
            \r\n\
        ";

        assert_eq!(
            headers(&raw_invalid[..]),
            Err(Err::Error((&b"\r\n"[..], ErrorKind::Verify)))
        );

        let raw = b"\
            content-length: 42\r\n\
            foo: is fantastic\r\n\
            bar: is beautiful\r\n\
            baz: is bananas\r\n\
            \r\n\
        ";
        let expected_headers: WarcHeadersParsed = vec![
            WarcHeaderParsed {
                token: "content-length",
                value: b"42",
                delim_left: b"",
                delim_right: b" ",
            },
            WarcHeaderParsed {
                token: "foo",
                value: b"is fantastic",
                delim_left: b"",
                delim_right: b" ",
            },
            WarcHeaderParsed {
                token: "bar",
                value: b"is beautiful",
                delim_left: b"",
                delim_right: b" ",
            },
            WarcHeaderParsed {
                token: "baz",
                value: b"is bananas",
                delim_left: b"",
                delim_right: b" ",
            },
        ];
        let expected_len = 42;

        assert_eq!(
            headers(&raw[..]),
            Ok((&b"\r\n"[..], (expected_headers, expected_len)))
        );
    }

    #[test]
    fn parse_record() {
        let raw = b"\
            WARC/1.0\r\n\
            Warc-Type: dunno\r\n\
            Content-Length: 5\r\n\
            \r\n\
            12345\r\n\
            \r\n\
        ";

        let expected = WarcRecordParsed {
            version: b"1.0",
            headers: vec![
                WarcHeaderParsed {
                    token: "Warc-Type",
                    value: b"dunno",
                    delim_left: b"",
                    delim_right: b" ",
                },
                WarcHeaderParsed {
                    token: "Content-Length",
                    value: b"5",
                    delim_left: b"",
                    delim_right: b" ",
                },
            ],
            body: b"12345",
        };

        assert_eq!(record(&raw[..]), Ok((&b""[..], expected)));
    }
}
