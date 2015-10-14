extern crate num;

pub use self::parser::{Parser, Token, Encoding, Class, Tag};
pub mod types;
pub mod dsl;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Error {
	BufferTooShort,
	InvalidMultipartTag,
	MultipartTagOverflow,
	MalformedToken,
	TokenTooLong,
	NestedTooDeep,
	TokenMismatch{file: &'static str, line: u32, col: u32},
	OutOfMemory,
	UnsupportedString,
	UnsupportedOid,
	PrematureEof
}

mod parser;
