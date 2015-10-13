mod iter;
mod stack;

macro_rules! try_opt {
	($expr:expr, $err:path) => (match $expr {
		::std::option::Option::Some(val) => val,
		::std::option::Option::None => return Err($err)
	})
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ParseError {
	BufferTooShort,
	InvalidMultipartTag,
	MultipartTagOverflow,
	MalformedToken,
	TokenTooLong,
	NestedTooDeep
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Class {
	Universal,
	Application,
	Context,
	Private
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Encoding {
	Primitive,
	Constructed
}

#[derive(Debug, PartialEq)]
pub enum Tag {
	Eoc,
	Bool,
	Int,
	Bitstring,
	Octetstring,
	Null,
	Oid,
	Utf8String,
	Sequence,
	Set,
	PrintableString,
	T61String,
	Ia5String,
	UtcTime,
	GeneralizedTime,
	VisibleString,
	Composed(Class, usize)
}

const TAG_CLASS_MASK: u8     = 0b11000000;
const TAG_ENCODING_MASK: u8  = 0b00100000;
const TAG_ID_MASK: u8        = 0b00011111;

const MULTIPART_MASK: u8     = 0b01111111;
const MULTIPART_SHIFT: usize = 128;
const MULTIPART_ID: usize    = 31;

impl Tag
{
	// 8.1.2
	fn from_bytes<'a, I>(iter: &mut I) -> Result<(Encoding, Tag), ParseError>
		where I: Iterator<Item=&'a u8>
	{
		use self::Class::*;
		use self::Encoding::*;
		use self::Tag::*;

		let byte = try_opt!(iter.next(), ParseError::BufferTooShort);

		let class = match (byte & TAG_CLASS_MASK) >> 6 {
			0 => Universal,
			1 => Application,
			2 => Context,
			3 => Private,
			_ => unreachable!()
		};

		let encoding = match (byte & TAG_ENCODING_MASK) >> 5 {
			0 => Primitive,
			1 => Constructed,
			_ => unreachable!()
		};

		let id = match (byte & TAG_ID_MASK) as usize {
			MULTIPART_ID => try!(Self::read_multipart_tag(iter)),
			id => id
		};

		Ok((encoding, match class {
			Universal => match id {
				 0 => Eoc,
				 1 => Bool,
				 2 => Int,
				 3 => Bitstring,
				 4 => Octetstring,
				 5 => Null,
				 6 => Oid,
				12 => Utf8String,
				16 => Sequence,
				17 => Set,
				19 => PrintableString,
				20 => T61String,
				22 => Ia5String,
				23 => UtcTime,
				24 => GeneralizedTime,
				26 => VisibleString,
				 _ => Composed(Universal, id),
			},
			_ => Composed(class, id)
		}))
	}

	// 8.1.2.4.2
	fn read_multipart_tag<'a, I>(iter: &mut I) -> Result<usize, ParseError>
		where I: Iterator<Item=&'a u8>
	{
		use self::ParseError::*;

		let mut tag = 0 as usize;
		for byte in iter {
			// Leading tag bytes must not be 0 under DER rules
			if tag == 0 && *byte == !MULTIPART_MASK {
				return Err(InvalidMultipartTag)
			}

			// Equivalent to tag = (tag << 7) | (byte & 0b01111111)
			tag = match tag.checked_mul(MULTIPART_SHIFT) {
				Some(t) => t | (*byte & MULTIPART_MASK) as usize,
				None => return Err(MultipartTagOverflow)
			};

			if byte & !MULTIPART_MASK == 0 {
				if tag < MULTIPART_ID {
					// Could have been encoded as a simple tag
					return Err(InvalidMultipartTag)
				} else {
					return Ok(tag)
				}
			};
		}

		Err(BufferTooShort)
	}
}

#[derive(Debug)]
pub struct Token<'a>{
	pub enc: Encoding,
	pub tag: Tag,
	pub depth: u8,
	pub header: &'a [u8],
	pub body: &'a [u8],
}

const LENGTH_MASK: u8 = 0b01111111;
const MIN_LONG_LENGTH: usize = 128;

impl<'a> Token<'a> {
	fn from_bytes<'b>(iter: &mut iter::Iter<'b>, depth: u8) -> Result<Token<'b>, ParseError>
	{
		use self::ParseError::*;

		let hdr_start = iter.pos();

		let (encoding, tag) = try!(Tag::from_bytes(iter));

		// Length (8.1.3)
		let length = {
			let byte = try_opt!(iter.next(), BufferTooShort);
			let length = byte & LENGTH_MASK;

			if (byte >> 7) == 1u8 {
				try!(Self::read_longform_length(length, iter))
			} else {
				length as usize
			}
		};

		let (min_remaining, _) = iter.size_hint();

		if min_remaining < length {
			return Err(BufferTooShort)
		}

		let pos = iter.pos();
		let header = iter.subslice(hdr_start, pos);
		let body = iter.subslice(pos, pos + length);

		Ok(Token{
			enc: encoding,
			tag: tag,
			depth: depth,
			header: header,
			body: body,
		})
	}

	fn read_longform_length<'b, I>(num_bytes: u8, iter: &mut I) -> Result<usize, ParseError>
		where I: Iterator<Item=&'b u8>
	{
		use self::ParseError::*;
		use std::mem::size_of;

		if num_bytes == 0 {
			// Indefinite form is forbidden (X.690 11/2008 8.1.3.6)
			return Err(MalformedToken)
		} else if num_bytes == 127 {
			// X.690 11/2008 item 8.1.3.5 (c)
			return Err(MalformedToken)
		} else if num_bytes as usize > size_of::<usize>() {
			return Err(TokenTooLong);
		}

		let mut length: usize = 0;
		for _ in 0..num_bytes {
			let byte = *try_opt!(iter.next(), BufferTooShort) as usize;

			length = (length << 8) | byte;
		}

		// 10.1
		if length < MIN_LONG_LENGTH {
			return Err(MalformedToken)
		}

		Ok(length)
	}
}

pub struct Parser<'a> {
	iter: iter::Iter<'a>,
	err: Option<ParseError>,
	stack: stack::FixedStack
}

impl<'a> Parser<'a> {
	pub fn new(bytes: &'a [u8]) -> Parser<'a> {
		Parser {
			iter: iter::Iter::new(bytes),
			err: None,
			stack: stack::FixedStack::new()
		}
	}

	fn parse(&mut self) -> Result<Token<'a>, ParseError> {
		use self::ParseError::*;
		use self::Encoding::*;

		let token = try!(Token::from_bytes(&mut self.iter, self.stack.depth()));
		let token_end = self.iter.pos() + token.body.len();

		// If this is not a root token, make sure it fits within its parent
		if let Some(parent_end) = self.stack.peek() {
			if token_end > parent_end {
				return Err(MalformedToken);
			}
		}

		match token {
			Token{enc: Primitive, body, ..} => {
				// Skip contents for primitive tokens
				for _ in 0..body.len() {
					if let None = self.iter.next() {
						return Err(BufferTooShort);
					}
				}
			},
			Token{enc: Constructed, ..} => {
				if let Err(_) = self.stack.push(token_end) {
					return Err(NestedTooDeep)
				}
			}
		}

		let pos = self.iter.pos();

		// Discard parent tokens which end at this position
		while let Some(parent_end) = self.stack.peek() {
			if pos == parent_end {
				self.stack.discard();
			} else {
				break;
			}
		}

		Ok(token)
	}
}

impl<'a> Iterator for Parser<'a> {
	type Item = Result<Token<'a>, ParseError>;

	fn next(&mut self) -> Option<Result<Token<'a>, ParseError>> {
		if let Some(_) = self.err {
			return None;
		}
 
		if self.iter.peek().is_none() {
			return None;
		}

		match self.parse() {
			Err(why) => {
				self.err = Some(why);
				Some(Err(why))
			},
			ok => Some(ok)
		}
	}
}

#[cfg(test)]
mod tests;
