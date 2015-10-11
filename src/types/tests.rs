use {Token, Encoding, Type};
use types::{String, Oid, StaticOid};
use types::TypeError::*;

#[test]
fn utf8string() {
	let token = Token{
		enc: Encoding::Primitive,
		ty: Type::Utf8String,
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
		ty: Type::VisibleString,
		depth: 0,
		header: &[0b0],
		body: "Unsupported characters: \x07\x00\x10".as_bytes(),
	};

	assert_eq!(String::from_token(&token).unwrap_err(), Unsupported);
}

#[test]
fn printablestring() {
	let token = Token{
		enc: Encoding::Primitive,
		ty: Type::PrintableString,
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
		ty: Type::PrintableString,
		depth: 0,
		header: &[0b0],
		body: "Unsupported characters: *;<>@".as_bytes(),
	};

	assert_eq!(String::from_token(&token).unwrap_err(), Malformed);
}

#[test]
fn constructed_string() {
	let token = Token{
		enc: Encoding::Constructed,
		ty: Type::Utf8String,
		depth: 0,
		header: &[0b0],
		body: "äöüß·".as_bytes(),
	};

	assert_eq!(String::from_token(&token).unwrap_err(), Malformed);
}

static MYOID: StaticOid = oid![2,2,11136];

#[test]
fn oid() {
	let token = Token{
		enc: Encoding::Primitive,
		ty: Type::Oid,
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
		ty: Type::Oid,
		depth: 0,
		header: &[0b0],
		body: &[0b0_1010010, 0b1_1010111, 0b0_0000000]
	};

	assert_eq!(Oid::from_token(&token).unwrap_err(), Malformed);
}

#[test]
fn truncated_oid() {
	let token = Token{
		enc: Encoding::Primitive,
		ty: Type::Oid,
		depth: 0,
		header: &[0b0],
		body: &[0b0_1010010, 0b1_1010111, 0b1_0000000]
	};

	assert_eq!(Oid::from_token(&token).unwrap_err(), Malformed);
}

#[test]
fn empty_oid() {
	let token = Token{
		enc: Encoding::Primitive,
		ty: Type::Oid,
		depth: 0,
		header: &[0b0],
		body: &[]
	};

	assert_eq!(Oid::from_token(&token).unwrap_err(), Malformed);
}
