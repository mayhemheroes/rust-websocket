use hyper::header::{Header, HeaderFormat};
use hyper::header::parsing::from_one_raw_str;
use std::fmt;
use std::ops::Deref;

/// Represents an Origin header
#[derive(PartialEq, Clone, Show)]
pub struct Origin(pub String);

impl Deref for Origin {
	type Target = String;
    fn deref<'a>(&'a self) -> &'a String {
        &self.0
    }
}

impl Header for Origin {
	fn header_name(_: Option<Origin>) -> &'static str {
		"Origin"
	}

	fn parse_header(raw: &[Vec<u8>]) -> Option<Origin> {
		from_one_raw_str(raw).map(|s| Origin(s))
	}
}

impl HeaderFormat for Origin {
	fn fmt_header(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		let Origin(ref value) = *self;
        write!(fmt, "{}", value)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_origin() {
		use header::Headers;
		
		let origin = Origin("foo bar".to_string());
		let mut headers = Headers::new();
		headers.set(origin);
		
		assert_eq!(&headers.to_string()[], "Origin: foo bar\r\n");
	}
}