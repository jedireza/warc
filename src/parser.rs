use crate::{WarcHeaders, WarcRecord};
use nom::{
    bytes::streaming::{tag, take, take_while1},
    character::streaming::{line_ending, not_line_ending, space0},
    error::ErrorKind,
    multi::many1,
    sequence::{delimited, tuple},
    IResult,
};
use std::str;

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

fn header_pair(input: &[u8]) -> IResult<&[u8], (&[u8], &[u8])> {
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

fn headers(input: &[u8]) -> IResult<&[u8], (WarcHeaders, usize)> {
    let (input, pairs) = many1(header_pair)(input)?;

    let mut content_length: Option<usize> = None;
    let mut headers: WarcHeaders = Vec::with_capacity(pairs.len());

    for pair in pairs {
        let key_str = match str::from_utf8(pair.0) {
            Err(_) => {
                return Err(nom::Err::Error((input, ErrorKind::Verify)));
            }
            Ok(key) => key,
        };

        if content_length == None && key_str.to_lowercase() == "content-length" {
            let value_str = match str::from_utf8(pair.1) {
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

        headers.push((key_str, pair.1));
    }

    if content_length == None {
        content_length = Some(0);
    }

    Ok((input, (headers, content_length.unwrap())))
}

pub fn record(input: &[u8]) -> IResult<&[u8], WarcRecord> {
    let (input, (version, headers, _)) = tuple((version, headers, line_ending))(input)?;
    let (input, (body, _, _)) = tuple((take(headers.1), line_ending, line_ending))(input)?;

    let record = WarcRecord {
        version: version,
        headers: headers.0,
        body: body,
    };

    Ok((input, record))
}

#[cfg(test)]
mod tests {
    use super::{header_pair, headers, record, version};
    use crate::{WarcHeaders, WarcRecord};
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
            header_pair(&b"some-header: all/the/things\r\n"[..]),
            Ok((&b""[..], (&b"some-header"[..], &b"all/the/things"[..])))
        );

        assert_eq!(
            header_pair(&b"another-header : with extra spaces\n"[..]),
            Ok((
                &b""[..],
                (&b"another-header"[..], &b"with extra spaces"[..])
            ))
        );

        assert_eq!(
            header_pair(&b"incomplete-header : missing-line-ending"[..]),
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
        let expected_headers: WarcHeaders = vec![
            ("content-length", b"42"),
            ("foo", b"is fantastic"),
            ("bar", b"is beautiful"),
            ("baz", b"is bananas"),
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

        let expected = WarcRecord {
            version: b"1.0",
            headers: vec![("Warc-Type", b"dunno"), ("Content-Length", b"5")],
            body: b"12345",
        };

        assert_eq!(record(&raw[..]), Ok((&b""[..], expected)));
    }
}
