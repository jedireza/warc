use http::header::HeaderMap;
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{line_ending, not_line_ending, space0},
    multi::many1,
    sequence::{delimited, tuple},
    IResult,
};

fn version(input: &str) -> IResult<&str, &str> {
    delimited(tag("WARC/"), not_line_ending, line_ending)(input)
}

fn is_header_token_char(chr: char) -> bool {
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

fn header_pair(input: &str) -> IResult<&str, (&str, &str)> {
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

// fn headers(input: &str) -> IResult<&str, HeaderMap> {
//     let (input, pairs) = many1(header_pair)(input)?;
//
//     let mut headers = HeaderMap::new();
//     for pair in pairs {
//         headers.insert(pair.0, pair.1.to_string().parse().unwrap());
//     }
//
//     Ok((input, headers))
// }

#[cfg(test)]
mod tests {
    use super::{header_pair, version};
    use nom::error::ErrorKind;
    use nom::Err;

    #[test]
    fn parse_version() {
        assert_eq!(version(&"WARC/0.0\r\n"[..]), Ok((&""[..], "0.0")));

        assert_eq!(version(&"WARC/1.0\r\n"[..]), Ok((&""[..], "1.0")));

        assert_eq!(
            version(&"WARC/2.0-alpha\r\n"[..]),
            Ok((&""[..], "2.0-alpha"))
        );
    }

    #[test]
    fn parse_header_pair() {
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
}
