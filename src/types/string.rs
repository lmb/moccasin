use std::str;
use std::convert;

use Token;
use Type::*;
use TypeError;
use TypeError::*;

impl convert::From<str::Utf8Error> for TypeError {
	fn from(_: str::Utf8Error) -> TypeError {
		TypeError::Malformed
	}
}

pub struct String<'a>(pub &'a str);

impl<'a> String<'a> {
	pub fn from_token(token: &'a Token) -> Result<String<'a>, TypeError> {
		let &Token{ref ty, body, ..} = token;

		match ty {
			&Utf8String      => Ok(String(try!{str::from_utf8(body)})),
			&PrintableString => Self::printable_string(body),
			&Ia5String       => Self::ascii_string(body),
			&VisibleString   => Self::ascii_string(body),
			&T61String       => Self::ascii_string(body),
			_ => Err(TypeMismatch)
		}
	}

	fn printable_string(body: &'a [u8]) -> Result<String<'a>, TypeError> {
		// Allowed characters are ( to z, excluding *, ;, <, >, @
		for byte in body {
			match *byte as char {
				'*' | ';' | '<' | '>' | '@' => return Err(Malformed),
				'(' ... 'z' | ' '           => continue,
				_                           => return Err(Malformed)
			}
		}

		Ok(String(try!(str::from_utf8(body))))
	}

	fn ascii_string(body: &'a [u8]) -> Result<String<'a>, TypeError> {
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
				_              => return Err(Unsupported),
			}
		}

		Ok(String(try!(str::from_utf8(body))))
	}
}
