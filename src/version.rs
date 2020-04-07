use std::fmt;

#[derive(Debug, PartialEq)]
pub struct WarcVersion<'a>(pub &'a str);

impl<'a> fmt::Display for WarcVersion<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WARC/{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::WarcVersion;

    #[test]
    fn display() {
        assert_eq!("WARC/0.0".to_owned(), format!("{}", WarcVersion("0.0")));
        assert_eq!("WARC/0.17", format!("{}", WarcVersion("0.17")));
        assert_eq!("WARC/1.0", format!("{}", WarcVersion("1.0")));
        assert_eq!("WARC/1.234", format!("{}", WarcVersion("1.234")));
        assert_eq!("WARC/2.0-alpha", format!("{}", WarcVersion("2.0-alpha")));
    }
}
