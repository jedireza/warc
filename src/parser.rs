use crate::{WarcHeaders, WarcRecord};
use nom::{
    bytes::complete::{tag, take, take_while1},
    character::complete::{line_ending, not_line_ending, space0},
    error::ErrorKind,
    multi::many1,
    sequence::{delimited, tuple},
    IResult,
};

pub fn version(input: &str) -> IResult<&str, &str> {
    delimited(tag("WARC/"), not_line_ending, line_ending)(input)
}

pub fn is_header_token_char(chr: char) -> bool {
    match chr as u8 {
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

pub fn header_pair(input: &str) -> IResult<&str, (&str, &str)> {
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

pub fn headers(input: &str) -> IResult<&str, (WarcHeaders, usize)> {
    let (input, pairs) = many1(header_pair)(input)?;

    let mut content_length: Option<usize> = None;
    let mut headers: WarcHeaders = Vec::with_capacity(pairs.len());
    for pair in pairs {
        if content_length == None && pair.0.to_lowercase() == "content-length" {
            match pair.1.parse::<usize>() {
                Err(_) => {
                    return Err(nom::Err::Error((input, ErrorKind::Verify)));
                }
                Ok(len) => {
                    content_length = Some(len);
                }
            }
        }

        headers.push((pair.0, pair.1));
    }

    if content_length == None {
        content_length = Some(0);
    }

    Ok((input, (headers, content_length.unwrap())))
}

pub fn record(input: &str) -> IResult<&str, WarcRecord> {
    let (input, (version, headers, _)) = tuple((version, headers, line_ending))(input)?;
    let (input, (body, _, _)) = tuple((take(headers.1), line_ending, line_ending))(input)?;
    let record = WarcRecord {
        version: version,
        headers: headers.0,
        body: body.as_bytes(),
    };

    Ok((input, record))
}

#[cfg(test)]
mod tests {
    use super::{header_pair, headers, record, version};
    use crate::{WarcHeaders, WarcRecord};
    use nom::error::ErrorKind;
    use nom::Err;

    #[test]
    fn version_parsing() {
        assert_eq!(version(&"WARC/0.0\r\n"[..]), Ok((&""[..], "0.0")));

        assert_eq!(version(&"WARC/1.0\r\n"[..]), Ok((&""[..], "1.0")));

        assert_eq!(
            version(&"WARC/2.0-alpha\r\n"[..]),
            Ok((&""[..], "2.0-alpha"))
        );
    }

    #[test]
    fn header_pair_parsing() {
        assert_eq!(
            header_pair(&"some-header: all/the/things\r\n"[..]),
            Ok((&""[..], (&"some-header"[..], &"all/the/things"[..])))
        );

        assert_eq!(
            header_pair(&"another-header : with extra spaces\n"[..]),
            Ok((&""[..], (&"another-header"[..], &"with extra spaces"[..])))
        );

        assert_eq!(
            header_pair(&"incomplete-header : missing-line-ending"[..]),
            Err(Err::Error((&""[..], ErrorKind::CrLf)))
        );
    }

    #[test]
    fn headers_parsing() {
        let expected_headers: WarcHeaders = vec![
            ("content-length", "42"),
            ("foo", "is fantastic"),
            ("bar", "is beautiful"),
            ("baz", "is bananas"),
        ];
        let expected_len = 42;

        assert_eq!(
            headers(&"content-length: 42\r\nfoo: is fantastic\r\nbar: is beautiful\r\nbaz: is bananas\r\n"[..]),
            Ok((&""[..], (expected_headers, expected_len)))
        );
    }

    #[test]
    fn parse_record() {
        let raw = "\
            WARC/1.0\r\n\
            Warc-Type: dunno\r\n\
            Content-Length: 5\r\n\
            \r\n\
            12345\r\n\
            \r\n\
        ";

        let expected = WarcRecord {
            version: "1.0",
            headers: vec![("Warc-Type", "dunno"), ("Content-Length", "5")],
            body: b"12345",
        };

        assert_eq!(record(&raw[..]), Ok((&""[..], expected)));
    }
}
