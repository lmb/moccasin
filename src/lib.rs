extern crate num;

pub use self::parser::{Parser, Token, Encoding, Class, Tag};
pub mod types;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Error {
	BufferTooShort,
	InvalidMultipartTag,
	MultipartTagOverflow,
	MalformedToken,
	TokenTooLong,
	NestedTooDeep,
	TokenMismatch,
	OutOfMemory,
	UnsupportedString,
	UnsupportedOid,
	PrematureEof
}

mod parser;
