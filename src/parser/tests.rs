use super::*;
use super::Encoding::*;
use super::Class::*;
use super::Tag::*;
use super::ParseError::*;
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
	let data = [
		0b00_1_10000u8, 0b0_0001000,
			0b00_0_00101, 0b0_0000000
	];

	let mut p = Parser::new(&data);

	let why = p.parse().unwrap_err();
	assert_eq!(why, BufferTooShort);
}
