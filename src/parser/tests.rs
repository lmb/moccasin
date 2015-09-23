use super::Encoding::*;
use super::Class::*;
use super::Type::*;
use super::Type;
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
		match Type::from_bytes(&mut Iter::new(case.0)) {
			Ok((encoding, ty)) => {
				assert_eq!(encoding, case.1);
				assert_eq!(ty, case.2);
			},
			Err(e) => panic!("{:?} for case {:?}", e, case)
		}
	}
}

#[test]
fn missing_multipart_definition()
{
	assert_eq!(Type::from_bytes(&mut Iter::new(&[
		0b11011111u8
	])), Err(BufferTooShort))
}

#[test]
fn invalid_multipart_definition()
{
	assert_eq!(Type::from_bytes(&mut Iter::new(&[
		0b11011111u8, 0b10000001
	])), Err(BufferTooShort))
}

#[test]
fn padded_multipart()
{
	// Extraneous leading zero
	assert_eq!(Type::from_bytes(&mut Iter::new(&[
		0b11011111u8, 0b10000000, 0b01111111
	])), Err(InvalidMultipartTag))
}

#[test]
fn simple_as_multipart()
{
	// Could have been encoded as simple tag
	assert_eq!(Type::from_bytes(&mut Iter::new(&[
		0b11011111u8, 0b00000111
	])), Err(InvalidMultipartTag))
}

#[test]
fn overflow_multipart()
{
	// Overflows tag size (machine dependent, this works for <= 64bit)
	assert_eq!(Type::from_bytes(&mut Iter::new(&[
		0b11011111u8,
		0b11111111, 0b10000001, 0b10000001, 0b10000001, 0b10000001,
		0b10000001, 0b10000001, 0b10000001, 0b10000001, 0b00000001
	])), Err(MultipartTagOverflow))
}
