use std::str::from_utf8;
use std::num::ParseIntError;
use std::convert::From;

use {Token, Tag, Error};
use types::TokenType;
use Error::MalformedToken;
use Encoding;
use Tag::{UtcTime, GeneralizedTime};

use chrono::*;

pub type Time = DateTime<UTC>;

/// The exact length of a UTCTime token (YYMMDDHHMMSS'Z')
const UTCTIME_LENGTH: usize = 12 + 1;
/// The exact length of a GeneralizedTime token (YYYYMMDDHHMMSS'Z')
const GENERALIZEDTIME_LENGTH: usize = 14 + 1;

impl From<ParseIntError> for Error {
	fn from(_: ParseIntError) -> Error {
		return MalformedToken;
	}
}

impl<'a> TokenType<'a> for Time {
	fn matches(tag: Tag) -> bool {
		match tag {
			Tag::UtcTime         |
			Tag::GeneralizedTime => true,
			_                    => false
		}
	}

	fn encoding() -> Encoding {
		Encoding::Primitive
	}

	fn from_token(token: &Token<'a>) -> Result<Time, Error> {
		if token.body[token.body.len()-1] != 'Z' as u8 {
			return Err(MalformedToken);
		}

		let s = match from_utf8(token.body) {
			Ok(s) => s,
			_     => return Err(MalformedToken)
		};

		let time = if token.tag == UtcTime {
			if token.body.len() != UTCTIME_LENGTH {
				return Err(MalformedToken)
			}

			let mut yr: u32 = try!((&s[ 0.. 2]).parse());
			let     mo: u32 = try!((&s[ 2.. 4]).parse());
			let     dy: u32 = try!((&s[ 4.. 6]).parse());
			let     h:  u32 = try!((&s[ 6.. 8]).parse());
			let     m:  u32 = try!((&s[ 8..10]).parse());
			let     s:  u32 = try!((&s[10..12]).parse());

			// Years are from (19)50 to (20)49, so 99 is 1999 and 00 is 2000.
			// Normalize years, since the encoding is not linear:
			// 00 -> 2000, 49 -> 2049, 50 -> 1950, 99 -> 1999
			yr += match yr > 49 {
				true  => 1900,
				false => 2000
			};

			UTC.ymd_opt(yr as i32, mo, dy).and_hms_opt(h, m, s)
		} else if token.tag == GeneralizedTime {
			if token.body.len() != GENERALIZEDTIME_LENGTH {
				return Err(MalformedToken)
			}

			let yr: u32 = try!((&s[ 0.. 4]).parse());
			let mo: u32 = try!((&s[ 4.. 6]).parse());
			let dy: u32 = try!((&s[ 6.. 8]).parse());
			let h:  u32 = try!((&s[ 8..10]).parse());
			let m:  u32 = try!((&s[10..12]).parse());
			let s:  u32 = try!((&s[12..14]).parse());

			UTC.ymd_opt(yr as i32, mo, dy).and_hms_opt(h, m, s)
		} else {
			return Err(MalformedToken);
		};

		match time {
			LocalResult::Single(time) => Ok(time),
			_                         => Err(MalformedToken)
		}
	}
}
