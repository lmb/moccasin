use std::str;
use std::convert;

use {Token, Encoding, Tag, Error};
use types::FromToken;
use Error::*;

impl convert::From<str::Utf8Error> for Error {
	fn from(_: str::Utf8Error) -> Error {
		MalformedToken
	}
}

#[derive(Debug)]
pub struct String<'a>(pub &'a str);

impl<'a> FromToken<'a> for String<'a> {
	fn from_token(token: &Token<'a>) -> Result<String<'a>, Error> {
		if token.enc != Encoding::Primitive {
			return Err(MalformedToken)
		}

		match token.tag {
			Tag::Utf8String      => Ok(String(try!{str::from_utf8(token.body)})),
			Tag::PrintableString => Self::printable_string(token.body),
			Tag::Ia5String       => Self::ascii_string(token.body),
			Tag::VisibleString   => Self::ascii_string(token.body),
			Tag::T61String       => Self::ascii_string(token.body),
			_ => Err(UnsupportedString)
		}
	}
}

impl<'a> String<'a> {
	fn printable_string(body: &'a [u8]) -> Result<String<'a>, Error> {
		// Allowed characters are ( to z, excluding *, ;, <, >, @
		for byte in body {
			match *byte as char {
				'*' | ';' | '<' | '>' | '@' => return Err(MalformedToken),
				'(' ... 'z' | ' '           => continue,
				_                           => return Err(MalformedToken)
			}
		}

		Ok(String(try!(str::from_utf8(body))))
	}

	fn ascii_string(body: &'a [u8]) -> Result<String<'a>, Error> {
		/* Strictly speaking, control codes are allowed for Ia5String,
		 * but since we don't have a way of dealing with code-page
		 * switching we restrict the type. This is non-conformant to the
		 * spec. Same goes for T61String, which can switch code pages
		 * mid-stream. We assume that the initial code-page is #6
		 * (ASCII), and flag switching as an error.
		 */

		for byte in body {
			match *byte as char {
				' ' ... '\x7f' => continue,
				_              => return Err(UnsupportedString),
			}
		}

		Ok(String(try!(str::from_utf8(body))))
	}
}

impl<'a> PartialEq<&'static str> for String<'a> {
	fn eq(&self, other: &&'static str) -> bool {
		self.0 == *other
	}
}
