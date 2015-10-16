use std::iter::Peekable;

use {Token, Tag, Error};
use Error::{TokenMismatch, PrematureEof};
use types::TokenType;

pub struct Matcher {
	tag: Option<Tag>,
	depth: u8
}

impl Matcher {
	pub fn new(depth: u8) -> Matcher {
		Matcher {
			depth: depth,
			tag: None
		}
	}

	pub fn with_tag(depth: u8, tag: Tag) -> Matcher {
		Matcher {
			depth: depth,
			tag: Some(tag)
		}
	}

	pub fn required<'a, I, T>(self, parser: &mut I) -> Result<T, Error>
		where I: Iterator<Item=Result<Token<'a>, Error>>, T: TokenType<'a>
	{
		self.required_at(parser, "<unknown>", 0, 0)
	}

	pub fn required_at<'a, I, T>(self, parser: &mut I, file: &'static str, line: u32, col: u32) -> Result<T, Error>
		where I: Iterator<Item=Result<Token<'a>, Error>>, T: TokenType<'a>
	{
		match parser.next() {
			Some(Ok(token)) => {
				match self.matches::<T>(&token) {
					true  => Ok(try!(T::from_token(&token))),
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

	pub fn optional<'a, I, T>(self, parser: &mut Peekable<I>) -> Option<T>
		where I: Iterator<Item=Result<Token<'a>, Error>>, T: TokenType<'a>
	{
		let matches = match parser.peek() {
			Some(&Ok(ref token)) => self.matches::<T>(token),
			_ => return None
		};

		if matches {
			return match T::from_token(&parser.next().unwrap().unwrap()) {
				Ok(ty) => Some(ty),
				_      => None
			}
		}

		None
	}

	fn matches<'a, T>(&self, token: &Token) -> bool
		where T: TokenType<'a>
	{
		token.depth == self.depth    &&
		token.enc   == T::encoding() &&
		match self.tag {
			Some(tag) => token.tag == tag,
			None      => T::matches(token.tag)
		}
	}
}

#[macro_export]
macro_rules! req {
	( $p:expr => $depth:expr, $ty:ty > ($class:expr, $id:expr) ) => {{
		let tag = $crate::Tag::Composed($class, $id);
		let matcher = $crate::dsl::Matcher::with_tag($depth, tag);

		if cfg!(debug_assertions) {
			try!(matcher.required_at::<_, $ty>($p, file!(), line!(), column!()))
		} else {
			try!(matcher.required::<_, $ty>($p))
		}
	}};
	( $p:expr => $depth:expr, $ty:ty ) => {{
		let matcher = $crate::dsl::Matcher::new($depth);

		if cfg!(debug_assertions) {
			try!(matcher.required_at::<_, $ty>($p, file!(), line!(), column!()))
		} else {
			try!(matcher.required::<_, $ty>($p))
		}
	}}
}

#[macro_export]
macro_rules! opt {
	( $p:expr => $depth:expr, $ty:ty > ($class:expr, $id:expr) ) => {{
		let tag = $crate::Tag::Composed($class, $id);
		$crate::dsl::Matcher::with_tag($depth, tag).optional::<_, $ty>($p)
	}};
	( $p:expr => $depth:expr, $ty:ty ) => {{
		$crate::dsl::Matcher::new($depth).optional::<_, $ty>($p)
	}}
}
