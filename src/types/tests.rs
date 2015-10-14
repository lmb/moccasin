use {Token, Encoding, Tag};
use types::{String, Oid, ConstOid, Int, Bitstring, Null, Bool, Time, FromToken};
use Error::*;

#[test]
fn utf8string() {
	let token = Token{
		enc: Encoding::Primitive,
		tag: Tag::Utf8String,
		depth: 0,
		header: &[],
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
		header: &[],
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
		header: &[],
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
		header: &[],
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
		header: &[],
		body: "äöüß·".as_bytes(),
	};

	assert_eq!(String::from_token(&token).unwrap_err(), MalformedToken);
}

static MYOID: ConstOid = oid![2,2,11136];

#[test]
fn oid() {
	let token = Token{
		enc: Encoding::Primitive,
		tag: Tag::Oid,
		depth: 0,
		header: &[],
		body: &[0b0_1010010, 0b1_1010111, 0b0_0000000]
	};

	let oid = Oid::from_token(&token).unwrap();
	assert_eq!(oid, MYOID);

	const MYOID2: ConstOid = oid![2,2,11136];
	assert_eq!(oid, MYOID2);
}

#[test]
fn constructed_oid() {
	let token = Token{
		enc: Encoding::Constructed,
		tag: Tag::Oid,
		depth: 0,
		header: &[],
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
		header: &[],
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
		header: &[],
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
		header: &[],
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
		header: &[],
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
		header: &[],
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
		header: &[],
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
		header: &[],
		body: &[0b11111111, 0b1_0000000]
	};

	assert_eq!(Int::<i32>::from_token(&token_ones).unwrap_err(), MalformedToken);

	let token_zeros = Token{
		enc: Encoding::Primitive,
		tag: Tag::Int,
		depth: 0,
		header: &[],
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
		header: &[],
		body: &[0b10000000, 0b00000000]
	};

	assert_eq!(Int::<i8>::from_token(&token).unwrap_err(), OutOfMemory);
}

#[test]
fn bool() {
	let true_tok = Token{
		enc: Encoding::Primitive,
		tag: Tag::Bool,
		depth: 0,
		header: &[],
		body: &[0xFF]
	};

	let Bool(b) = Bool::from_token(&true_tok).unwrap();
	assert_eq!(b, true);

	let false_tok = Token{
		enc: Encoding::Primitive,
		tag: Tag::Bool,
		depth: 0,
		header: &[],
		body: &[0x0]
	};

	let Bool(b) = Bool::from_token(&false_tok).unwrap();
	assert_eq!(b, false);
}

#[test]
fn invalid_bool() {
	let wrong_value = Token{
		enc: Encoding::Primitive,
		tag: Tag::Bool,
		depth: 0,
		header: &[],
		body: &[0xAB]
	};

	assert_eq!(Bool::from_token(&wrong_value).unwrap_err(), MalformedToken);

	let too_long = Token{
		enc: Encoding::Primitive,
		tag: Tag::Bool,
		depth: 0,
		header: &[],
		body: &[0x0, 0x0]
	};

	assert_eq!(Bool::from_token(&too_long).unwrap_err(), MalformedToken);

	let wrong_encoding = Token{
		enc: Encoding::Constructed,
		tag: Tag::Bool,
		depth: 0,
		header: &[],
		body: &[0x0]
	};

	assert_eq!(Bool::from_token(&wrong_encoding).unwrap_err(), MalformedToken);
}

#[test]
fn null() {
	let ok = Token{
		enc: Encoding::Primitive,
		tag: Tag::Null,
		depth: 0,
		header: &[],
		body: &[]
	};

	Null::from_token(&ok).unwrap();
}

#[test]
fn invalid_null() {
	let nonempty = Token{
		enc: Encoding::Primitive,
		tag: Tag::Null,
		depth: 0,
		header: &[],
		body: &[0xFF]
	};

	assert_eq!(Null::from_token(&nonempty).unwrap_err(), MalformedToken);

	let wrong_encoding = Token{
		enc: Encoding::Constructed,
		tag: Tag::Null,
		depth: 0,
		header: &[],
		body: &[]
	};

	assert_eq!(Null::from_token(&wrong_encoding).unwrap_err(), MalformedToken);
}

#[test]
fn bitstring() {
	let ok = Token{
		enc: Encoding::Primitive,
		tag: Tag::Bitstring,
		depth: 0,
		header: &[],
		body: &[0x4, 0xA3, 0xB4, 0xF0]
	};

	let r1 = Bitstring::from_token(&ok).unwrap();
	assert_eq!(r1.as_unsigned::<u32>(), Ok(0xA3B4F));

	let empty = Token{
		enc: Encoding::Primitive,
		tag: Tag::Bitstring,
		depth: 0,
		header: &[],
		body: &[0x0]
	};

	let r2 = Bitstring::from_token(&empty).unwrap();
	assert_eq!(r2.as_unsigned::<u16>(), Ok(0));
}

#[test]
fn invalid_bitstring() {
	let unused = Token{
		enc: Encoding::Primitive,
		tag: Tag::Bitstring,
		depth: 0,
		header: &[],
		body: &[0x4]
	};

	assert_eq!(Bitstring::from_token(&unused).unwrap_err(), MalformedToken);

	let empty = Token{
		enc: Encoding::Primitive,
		tag: Tag::Bitstring,
		depth: 0,
		header: &[],
		body: &[]
	};

	assert_eq!(Bitstring::from_token(&empty).unwrap_err(), MalformedToken);

	let wrong_encoding = Token{
		enc: Encoding::Constructed,
		tag: Tag::Bitstring,
		depth: 0,
		header: &[],
		body: &[0x0]
	};

	assert_eq!(Bitstring::from_token(&wrong_encoding).unwrap_err(), MalformedToken);
}

#[test]
fn invalid_bitstring_padding() {
	let long_unused = Token{
		enc: Encoding::Primitive,
		tag: Tag::Bitstring,
		depth: 0,
		header: &[],
		body: &[0xC, 0xA3, 0xB0, 0x00]
	};

	assert_eq!(Bitstring::from_token(&long_unused).unwrap_err(), MalformedToken);

	let extra_padding = Token{
		enc: Encoding::Primitive,
		tag: Tag::Bitstring,
		depth: 0,
		header: &[],
		body: &[0x0, 0xA3, 0xB0, 0x00]
	};

	assert_eq!(Bitstring::from_token(&extra_padding).unwrap_err(), MalformedToken);

	let nonzero_unused = Token{
		enc: Encoding::Primitive,
		tag: Tag::Bitstring,
		depth: 0,
		header: &[],
		body: &[0x4, 0xA3, 0xB0, 0xBF]
	};

	assert_eq!(Bitstring::from_token(&nonzero_unused).unwrap_err(), MalformedToken);
}

#[test]
fn time() {
	let utctime = Token{
		enc: Encoding::Primitive,
		tag: Tag::UtcTime,
		depth: 0,
		header: &[],
		body: "491020181001Z".as_bytes()
	};

	assert_eq!(Time::from_token(&utctime).unwrap().to_rfc3339(), "2049-10-20T18:10:01+00:00");

	let utctime_19thcent = Token{
		enc: Encoding::Primitive,
		tag: Tag::UtcTime,
		depth: 0,
		header: &[],
		body: "991020181001Z".as_bytes()
	};

	assert_eq!(Time::from_token(&utctime_19thcent).unwrap().to_rfc3339(), "1999-10-20T18:10:01+00:00");

	let generalized = Token{
		enc: Encoding::Primitive,
		tag: Tag::GeneralizedTime,
		depth: 0,
		header: &[],
		body: "20991020181001Z".as_bytes()
	};

	assert_eq!(Time::from_token(&generalized).unwrap().to_rfc3339(), "2099-10-20T18:10:01+00:00");
}

#[test]
fn invalid_time() {
	let invalid_encoding = Token{
		enc: Encoding::Constructed,
		tag: Tag::UtcTime,
		depth: 0,
		header: &[],
		body: "491020181001Z".as_bytes()
	};

	assert_eq!(Time::from_token(&invalid_encoding).unwrap_err(), MalformedToken);

	let invalid_time = Token{
		enc: Encoding::Primitive,
		tag: Tag::UtcTime,
		depth: 0,
		header: &[],
		body: "990231181001Z".as_bytes()
	};

	assert_eq!(Time::from_token(&invalid_time).unwrap_err(), MalformedToken);

	let truncated = Token{
		enc: Encoding::Primitive,
		tag: Tag::GeneralizedTime,
		depth: 0,
		header: &[],
		body: "20991020181001".as_bytes()
	};

	assert_eq!(Time::from_token(&truncated).unwrap_err(), MalformedToken);
}
