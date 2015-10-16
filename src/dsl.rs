use std::iter::Peekable;

use {Parser, Token, Tag, Encoding, Error};
use Tag::*;
use Encoding::*;
use Error::{TokenMismatch, PrematureEof};

pub struct Matcher {
	tag: Tag,
	depth: u8,
	enc: Encoding
}

impl Matcher {
	pub fn new(depth: u8, tag: Tag) -> Matcher {
		Matcher {
			depth: depth,
			enc: match tag {
				Sequence |
				Set      => Constructed,
				_        => Primitive
			},
			tag: tag
		}
	}

	pub fn encoding(mut self, enc: Encoding) -> Self {
		self.enc = enc;
		self
	}

	pub fn required<'a>(self, parser: &mut Peekable<Parser<'a>>) -> Result<Token<'a>, Error> {
		self.required_at(parser, "<unknown>", 0, 0)
	}

	pub fn required_at<'a>(self, parser: &mut Peekable<Parser<'a>>, file: &'static str, line: u32, col: u32) -> Result<Token<'a>, Error> {
		match parser.next() {
			Some(Ok(token)) => {
				match self.matches(&token) {
					true  => Ok(token),
					false => Err(TokenMismatch{
						file: file,
						line: line,
						col: col
					})
				}
			},
			Some(Err(why)) => Err(why),
			None => Err(PrematureEof)
		}
	}

	pub fn optional<'a>(self, parser: &mut Peekable<Parser<'a>>) -> Option<Token<'a>> {
		let matches = match parser.peek() {
			Some(&Ok(ref token)) => self.matches(token),
			_ => return None
		};

		match matches {
			true  => parser.next().unwrap().ok(),
			false => None
		}
	}

	fn matches(&self, token: &Token) -> bool {
		token.enc   == self.enc   &&
		token.depth == self.depth &&
		match self.tag {
			String => match token.tag {
				Ia5String       => true,
				VisibleString   => true,
				Utf8String      => true,
				PrintableString => true,
				_                    => false
			},
			Time => match token.tag {
				GeneralizedTime => true,
				UtcTime         => true,
				_                    => false
			},
			ref other => *other == token.tag
		}
	}
}

#[macro_export]
macro_rules! req {
	( $p:expr => $depth:expr, $tag:expr, $enc:expr; $ty:ty ) => {{
		let tok = req!{ $p => $depth, $tag, $enc };
		try!(<$ty>::from_token(&tok))
	}};
	( $p:expr => $depth:expr, $tag:expr; $ty:ty ) => {{
		let tok = req!{ $p => $depth, $tag };
		try!(<$ty>::from_token(&tok))
	}};
	( $p:expr => $depth:expr, $tag:expr, $enc:expr ) => {{
		let matcher = $crate::dsl::Matcher::new($depth, $tag).encoding($enc);

		if cfg!(debug_assertions) {
			try!(matcher.required_at($p, file!(), line!(), column!()))
		} else {
			try!(matcher.required($p))
		}
	}};
	( $p:expr => $depth:expr, $tag:expr ) => {{
		let matcher = $crate::dsl::Matcher::new($depth, $tag);

		if cfg!(debug_assertions) {
			try!(matcher.required_at($p, file!(), line!(), column!()))
		} else {
			try!(matcher.required($p))
		}
	}}
}

#[macro_export]
macro_rules! opt {
	( $p:expr => $depth:expr, $tag:expr, $enc:expr; $ty:ty ) => {{
		let tok = opt!{ $p => $depth, $tag, $enc };

		match tok {
			Some(tok) => Some(try!(<$ty>::from_token(&tok))),
			None => None
		}
	}};
	( $p:expr => $depth:expr, $tag:expr; $ty:ty ) => {{
		let tok = opt!{ $p => $depth, $tag };

		match tok {
			Some(tok) => Some(try!(<$ty>::from_token(&tok))),
			None => None
		}
	}};
	( $p:expr => $depth:expr, $tag:expr, $enc:expr ) => {{
		$crate::dsl::Matcher::new($depth, $tag).encoding($enc).optional($p)
	}};
	( $p:expr => $depth:expr, $tag:expr ) => {{
		$crate::dsl::Matcher::new($depth, $tag).optional($p)
	}}
}
