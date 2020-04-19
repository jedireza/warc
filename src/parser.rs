use crate::{WarcHeaderRef, WarcHeadersRef, WarcRecordRef};
use nom::{
    bytes::streaming::{tag, take, take_while1},
    character::streaming::{line_ending, not_line_ending, space0},
    error::ErrorKind,
    multi::many1,
    sequence::tuple,
    IResult,
};
use std::str;

// TODO: evaluate the use of `ErrorKind::Verify` here.
fn version(input: &[u8]) -> IResult<&[u8], &str> {
    let (input, (_, version, _)) = tuple((tag("WARC/"), not_line_ending, line_ending))(input)?;

    let version_str = match str::from_utf8(version) {
        Err(_) => {
            return Err(nom::Err::Error((input, ErrorKind::Verify)));
        }
        Ok(version) => version,
    };

    Ok((input, version_str))
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

fn header(input: &[u8]) -> IResult<&[u8], (&[u8], &[u8])> {
    let (input, (token, _, _, _, value, _)) = tuple((
        take_while1(is_header_token_char),
        space0,
        tag(":"),
        space0,
        not_line_ending,
        line_ending,
    ))(input)?;

    Ok((input, (token, value)))
}

// TODO: evaluate the use of `ErrorKind::Verify` here.
pub fn headers(input: &[u8]) -> IResult<&[u8], (&str, WarcHeadersRef, usize)> {
    let (input, version) = version(input)?;
    let (input, headers) = many1(header)(input)?;

    let mut content_length: Option<usize> = None;
    let mut warc_headers = WarcHeadersRef::with_capacity(headers.len());

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

        warc_headers.push(WarcHeaderRef {
            token: token_str,
            value: header.1,
        });
    }

    // TODO: Technically if we didn't find a `content-length` header, the record is invalid. Should
    // we be returning an error here instead?
    if content_length == None {
        content_length = Some(0);
    }

    Ok((input, (version, warc_headers, content_length.unwrap())))
}

pub fn record(input: &[u8]) -> IResult<&[u8], WarcRecordRef> {
    let (input, (headers, _)) = tuple((headers, line_ending))(input)?;
    let (input, (body, _, _)) = tuple((take(headers.2), line_ending, line_ending))(input)?;

    let record = WarcRecordRef {
        version: headers.0,
        headers: headers.1,
        body: body,
    };

    Ok((input, record))
}

#[cfg(test)]
mod tests {
    use super::{header, headers, record, version};
    use super::{WarcHeaderRef, WarcHeadersRef, WarcRecordRef};
    use nom::error::ErrorKind;
    use nom::Err;
    use nom::Needed;

    #[test]
    fn version_parsing() {
        assert_eq!(version(&b"WARC/0.0\r\n"[..]), Ok((&b""[..], &"0.0"[..])));

        assert_eq!(version(&b"WARC/1.0\r\n"[..]), Ok((&b""[..], &"1.0"[..])));

        assert_eq!(
            version(&b"WARC/2.0-alpha\r\n"[..]),
            Ok((&b""[..], &"2.0-alpha"[..]))
        );
    }

    #[test]
    fn header_pair_parsing() {
        assert_eq!(
            header(&b"some-header: all/the/things\r\n"[..]),
            Ok((&b""[..], (&b"some-header"[..], &b"all/the/things"[..],)))
        );

        assert_eq!(
            header(&b"another-header : with extra spaces\r\n"[..]),
            Ok((
                &b""[..],
                (&b"another-header"[..], &b"with extra spaces"[..],)
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
            WARC/1.0\r\n\
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
            WARC/1.0\r\n\
            content-length: 42\r\n\
            foo: is fantastic\r\n\
            bar: is beautiful\r\n\
            baz: is bananas\r\n\
            \r\n\
        ";
        let expected_version = "1.0";
        let expected_headers = WarcHeadersRef::new(vec![
            WarcHeaderRef {
                token: "content-length",
                value: b"42",
            },
            WarcHeaderRef {
                token: "foo",
                value: b"is fantastic",
            },
            WarcHeaderRef {
                token: "bar",
                value: b"is beautiful",
            },
            WarcHeaderRef {
                token: "baz",
                value: b"is bananas",
            },
        ]);
        let expected_len = 42;

        assert_eq!(
            headers(&raw[..]),
            Ok((
                &b"\r\n"[..],
                (expected_version, expected_headers, expected_len)
            ))
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
            WARC/1.0\r\n\
            Warc-Type: another\r\n\
            Content-Length: 6\r\n\
            \r\n\
            123456\r\n\
            \r\n\
        ";

        let expected = WarcRecordRef {
            version: "1.0",
            headers: WarcHeadersRef::new(vec![
                WarcHeaderRef {
                    token: "Warc-Type",
                    value: b"dunno",
                },
                WarcHeaderRef {
                    token: "Content-Length",
                    value: b"5",
                },
            ]),
            body: b"12345",
        };

        assert_eq!(
            record(&raw[..]),
            Ok((
                &b"WARC/1.0\r\nWarc-Type: another\r\nContent-Length: 6\r\n\r\n123456\r\n\r\n"[..],
                expected
            ))
        );
    }
}
