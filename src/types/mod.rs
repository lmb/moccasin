pub use self::string::String;
pub use self::oid::{Oid, ConstOid};
pub use self::int::Int;
pub use self::bool::Bool;
pub use self::null::Null;
pub use self::bitstring::Bitstring;
pub use self::time::Time;
pub use self::octetstring::Octetstring;

mod string;
#[macro_use]
mod oid;
mod int;
mod bool;
mod null;
mod bitstring;
mod time;
mod octetstring;

use {Token, Tag, Encoding, Error};

pub trait TokenType<'a> where Self: Sized {
	fn from_token(token: &Token<'a>) -> Result<Self, Error>;
	fn matches(tag: Tag) -> bool;
	fn encoding() -> Encoding;
}

pub struct Sequence;

impl<'a> TokenType<'a> for Sequence {
	fn matches(tag: Tag) -> bool {
		tag == Tag::Sequence
	}

	fn encoding() -> Encoding {
		Encoding::Constructed
	}

	fn from_token(_: &Token<'a>) -> Result<Self, Error> {
		Ok(Sequence)
	}
}

pub struct Set;

impl<'a> TokenType<'a> for Set {
	fn matches(tag: Tag) -> bool {
		tag == Tag::Set
	}

	fn encoding() -> Encoding {
		Encoding::Constructed
	}

	fn from_token(_: &Token<'a>) -> Result<Self, Error> {
		Ok(Set)
	}
}

#[cfg(test)]
mod tests;
