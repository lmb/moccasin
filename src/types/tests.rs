use {Token, Encoding, Tag};
use types::{String, Oid, StaticOid, Int};
use Error::*;

#[test]
fn utf8string() {
	let token = Token{
		enc: Encoding::Primitive,
		tag: Tag::Utf8String,
		depth: 0,
		header: &[0b0],
		body: "äöüß·".as_bytes(),
	};

	assert_eq!(String::from_token(&token).unwrap(), "äöüß·")
}

#[test]
fn unsupported_asciistring() {
	let token = Token{
		enc: Encoding::Primitive,
		tag: Tag::VisibleString,
		depth: 0,
		header: &[0b0],
		body: "Unsupported characters: \x07\x00\x10".as_bytes(),
	};

	assert_eq!(String::from_token(&token).unwrap_err(), UnsupportedString);
}

#[test]
fn printablestring() {
	let token = Token{
		enc: Encoding::Primitive,
		tag: Tag::PrintableString,
		depth: 0,
		header: &[0b0],
		body: "Western Cape".as_bytes(),
	};

	assert_eq!(String::from_token(&token).unwrap(), "Western Cape");
}

#[test]
fn invalid_printablestring() {
	let token = Token{
		enc: Encoding::Primitive,
		tag: Tag::PrintableString,
		depth: 0,
		header: &[0b0],
		body: "Unsupported characters: *;<>@".as_bytes(),
	};

	assert_eq!(String::from_token(&token).unwrap_err(), MalformedToken);
}

#[test]
fn constructed_string() {
	let token = Token{
		enc: Encoding::Constructed,
		tag: Tag::Utf8String,
		depth: 0,
		header: &[0b0],
		body: "äöüß·".as_bytes(),
	};

	assert_eq!(String::from_token(&token).unwrap_err(), MalformedToken);
}

static MYOID: StaticOid = oid![2,2,11136];

#[test]
fn oid() {
	let token = Token{
		enc: Encoding::Primitive,
		tag: Tag::Oid,
		depth: 0,
		header: &[0b0],
		body: &[0b0_1010010, 0b1_1010111, 0b0_0000000]
	};

	let oid = Oid::from_token(&token).unwrap();
	assert_eq!(oid, MYOID);
}

#[test]
fn constructed_oid() {
	let token = Token{
		enc: Encoding::Constructed,
		tag: Tag::Oid,
		depth: 0,
		header: &[0b0],
		body: &[0b0_1010010, 0b1_1010111, 0b0_0000000]
	};

	assert_eq!(Oid::from_token(&token).unwrap_err(), MalformedToken);
}

#[test]
fn truncated_oid() {
	let token = Token{
		enc: Encoding::Primitive,
		tag: Tag::Oid,
		depth: 0,
		header: &[0b0],
		body: &[0b0_1010010, 0b1_1010111, 0b1_0000000]
	};

	assert_eq!(Oid::from_token(&token).unwrap_err(), MalformedToken);
}

#[test]
fn empty_oid() {
	let token = Token{
		enc: Encoding::Primitive,
		tag: Tag::Oid,
		depth: 0,
		header: &[0b0],
		body: &[]
	};

	assert_eq!(Oid::from_token(&token).unwrap_err(), MalformedToken);
}

#[test]
fn int() {
	let token = Token{
		enc: Encoding::Primitive,
		tag: Tag::Int,
		depth: 0,
		header: &[0b0],
		body: &[0b101]
	};

	let Int(v) = Int::<i32>::from_token(&token).unwrap();
	assert_eq!(v, 5);
}

#[test]
fn negative_int() {
	let token = Token{
		enc: Encoding::Primitive,
		tag: Tag::Int,
		depth: 0,
		header: &[0b0],
		body: &[0b11111111]
	};

	let Int(v) = Int::<i32>::from_token(&token).unwrap();
	assert_eq!(v, -1);
}

#[test]
fn empty_int() {
	let token = Token{
		enc: Encoding::Primitive,
		tag: Tag::Int,
		depth: 0,
		header: &[0b0],
		body: &[]
	};

	assert_eq!(Int::<i32>::from_token(&token).unwrap_err(), MalformedToken);
}

#[test]
fn constructed_int() {
	let token = Token{
		enc: Encoding::Constructed,
		tag: Tag::Int,
		depth: 0,
		header: &[0b0],
		body: &[]
	};

	assert_eq!(Int::<i32>::from_token(&token).unwrap_err(), MalformedToken);
}

#[test]
fn invalid_int_padding() {
	let token_ones = Token{
		enc: Encoding::Primitive,
		tag: Tag::Int,
		depth: 0,
		header: &[0b0],
		body: &[0b11111111, 0b1_0000000]
	};

	assert_eq!(Int::<i32>::from_token(&token_ones).unwrap_err(), MalformedToken);

	let token_zeros = Token{
		enc: Encoding::Primitive,
		tag: Tag::Int,
		depth: 0,
		header: &[0b0],
		body: &[0b00000000, 0b0_0000000]
	};

	assert_eq!(Int::<i32>::from_token(&token_zeros).unwrap_err(), MalformedToken);
}

#[test]
fn decode_large_int() {
	let token = Token{
		enc: Encoding::Primitive,
		tag: Tag::Int,
		depth: 0,
		header: &[0b0],
		body: &[0b10000000, 0b00000000]
	};

	assert_eq!(Int::<i8>::from_token(&token).unwrap_err(), OutOfMemory);
}
