use {Tag, Parser, Token};
use Encoding::*;
use Class::*;
use Tag::*;
use Error::*;
use super::iter::Iter;

#[test]
fn decode_type()
{
	let cases = [
		(&[0b11001111u8] as &[u8], Primitive, Composed(Private, 0b1111)),
		(&[0b00100001u8], Constructed, Bool),
		(&[0b00011111u8, 0b10000001, 0b01111111], Primitive, Composed(Universal, 0b11111111))
	];

	for case in &cases {
		match Tag::from_bytes(&mut Iter::new(case.0)) {
			Ok((encoding, tag)) => {
				assert_eq!(encoding, case.1);
				assert_eq!(tag, case.2);
			},
			Err(e) => panic!("{:?} for case {:?}", e, case)
		}
	}
}

#[test]
fn missing_multipart_definition()
{
	assert_eq!(Tag::from_bytes(&mut Iter::new(&[
		0b11011111u8
	])), Err(BufferTooShort))
}

#[test]
fn invalid_multipart_definition()
{
	assert_eq!(Tag::from_bytes(&mut Iter::new(&[
		0b11011111u8, 0b10000001
	])), Err(BufferTooShort))
}

#[test]
fn padded_multipart()
{
	// Extraneous leading zero
	assert_eq!(Tag::from_bytes(&mut Iter::new(&[
		0b11011111u8, 0b10000000, 0b01111111
	])), Err(InvalidMultipartTag))
}

#[test]
fn simple_as_multipart()
{
	// Could have been encoded as simple tag
	assert_eq!(Tag::from_bytes(&mut Iter::new(&[
		0b11011111u8, 0b00000111
	])), Err(InvalidMultipartTag))
}

#[test]
fn overflow_multipart()
{
	// Overflows tag size (machine dependent, this works for <= 64bit)
	assert_eq!(Tag::from_bytes(&mut Iter::new(&[
		0b11011111u8,
		0b11111111, 0b10000001, 0b10000001, 0b10000001, 0b10000001,
		0b10000001, 0b10000001, 0b10000001, 0b10000001, 0b00000001
	])), Err(MultipartTagOverflow))
}

#[test]
fn malformed_nesting() {
	// The child token is longer than the parent.
	let data = [
		0b00_1_10000u8, 0b0_0000001,
			0b00_0_00100, 0b0_0000001, 0b11110000
	];

	let mut p = Parser::new(&data);

	let seq = p.parse().unwrap();
	assert!(seq.tag == Sequence);

	let oct = p.parse().unwrap_err();
	assert_eq!(oct, MalformedToken);
}

#[test]
fn truncated_token() {
	let data1 = [
		0b00_1_10000u8, 0b0_0001000,
			0b00_0_00101, 0b0_0000000
	];

	let mut p = Parser::new(&data1);

	assert_eq!(p.parse().unwrap_err(), BufferTooShort);

	let data2 = [
		0b00_0_00000u8, 0b0_00000010
	];

	let mut p = Parser::new(&data2);

	assert_eq!(p.parse().unwrap_err(), BufferTooShort);
}

#[test]
fn x509_google() {
	let data = include_bytes!("../../tests/certs/google.der");

	let tokens = [
		(Constructed, Sequence,             0, 902),
		(Constructed, Sequence,             1, 751),
		(Constructed, Composed(Context, 0), 2, 3),
		(Primitive,   Int,                  3, 1),
		(Primitive,   Int,                  2, 16),
		(Constructed, Sequence,             2, 13),
		(Primitive,   Oid,                  3, 9),
		(Primitive,   Null,                 3, 0),
		(Constructed, Sequence,             2, 206),
		(Constructed, Set,                  3, 11),
		(Constructed, Sequence,             4, 9),
		(Primitive,   Oid,                  5, 3),
		(Primitive,   PrintableString,      5, 2),
		(Constructed, Set,                  3, 21),
		(Constructed, Sequence,             4, 19),
		(Primitive,   Oid,                  5, 3),
		(Primitive,   PrintableString,      5, 12),
		(Constructed, Set,                  3, 18),
		(Constructed, Sequence,             4, 16),
		(Primitive,   Oid,                  5, 3),
		(Primitive,   PrintableString,      5, 9),
		(Constructed, Set,                  3, 29),
		(Constructed, Sequence,             4, 27),
		(Primitive,   Oid,                  5, 3),
		(Primitive,   PrintableString,      5, 20),
		(Constructed, Set,                  3, 40),
		(Constructed, Sequence,             4, 38),
		(Primitive,   Oid,                  5, 3),
		(Primitive,   PrintableString,      5, 31),
		(Constructed, Set,                  3, 33),
		(Constructed, Sequence,             4, 31),
		(Primitive,   Oid,                  5, 3),
		(Primitive,   PrintableString,      5, 24),
		(Constructed, Set,                  3, 40),
		(Constructed, Sequence,             4, 38),
		(Primitive,   Oid,                  5, 9),
		(Primitive,   Ia5String,            5, 25),
		(Constructed, Sequence,             2, 30),
		(Primitive,   UtcTime,              3, 13),
		(Primitive,   UtcTime,              3, 13),
		(Constructed, Sequence,             2, 102),
		(Constructed, Set,                  3, 11),
		(Constructed, Sequence,             4, 9),
		(Primitive,   Oid,                  5, 3),
		(Primitive,   PrintableString,      5, 2),
		(Constructed, Set,                  3, 19),
		(Constructed, Sequence,             4, 17),
		(Primitive,   Oid,                  5, 3),
		(Primitive,   PrintableString,      5, 10),
		(Constructed, Set,                  3, 22),
		(Constructed, Sequence,             4, 20),
		(Primitive,   Oid,                  5, 3),
		(Primitive,   PrintableString,      5, 13),
		(Constructed, Set,                  3, 19),
		(Constructed, Sequence,             4, 17),
		(Primitive,   Oid,                  5, 3),
		(Primitive,   PrintableString,      5, 10),
		(Constructed, Set,                  3, 21),
		(Constructed, Sequence,             4, 19),
		(Primitive,   Oid,                  5, 3),
		(Primitive,   Utf8String,           5, 12),
		(Constructed, Sequence,             2, 159),
		(Constructed, Sequence,             3, 13),
		(Primitive,   Oid,                  4, 9),
		(Primitive,   Null,                 4, 0),
		(Primitive,   Bitstring,            3, 141),
		(Constructed, Composed(Context, 3), 2, 203),
		(Constructed, Sequence,             3, 200),
		(Constructed, Sequence,             4, 29),
		(Primitive,   Oid,                  5, 3),
		(Primitive,   Octetstring,          5, 22),
		(Constructed, Sequence,             4, 64),
		(Primitive,   Oid,                  5, 3),
		(Primitive,   Octetstring,          5, 57),
		(Constructed, Sequence,             4, 50),
		(Primitive,   Oid,                  5, 8),
		(Primitive,   Octetstring,          5, 38),
		(Constructed, Sequence,             4, 35),
		(Primitive,   Oid,                  5, 3),
		(Primitive,   Octetstring,          5, 28),
		(Constructed, Sequence,             4, 12),
		(Primitive,   Oid,                  5, 3),
		(Primitive,   Bool,                 5, 1),
		(Primitive,   Octetstring,          5, 2),
		(Constructed, Sequence,             1, 13),
		(Primitive,   Oid,                  2, 9),
		(Primitive,   Null,                 2, 0),
		(Primitive,   Bitstring,            1, 129)
	];

	let p = Parser::new(data);

	for (token, expected) in p.zip(tokens.iter()) {
		match token {
			Ok(Token{enc, tag, depth, body, ..}) => {
				if depth != expected.2 {
					panic!("{:?} {:?} {:?} {:?}", enc, tag, depth, expected)
				}
				assert_eq!(enc,        expected.0);
				assert_eq!(tag,        expected.1);
				assert_eq!(depth,      expected.2);
				assert_eq!(body.len(), expected.3);
			},
			Err(e) => panic!("Expected {:?}, got {:?}", expected, e)
		}
	}
}
