use {Token, Encoding, Type, String};
use TypeError::*;

#[test]
fn utf8string() {
	let token = Token{
		enc: Encoding::Primitive,
		ty: Type::Utf8String,
		len: 0,
		header: &[0b0],
		body: "äöüß·".as_bytes(),
	};

	match String::from_token(&token) {
		Ok(String(s)) => assert_eq!(s, "äöüß·"),
		Err(why) => panic!("Could not parse String from Token: {:?}", why)
	}
}

#[test]
fn unsupported_asciistring() {
	let token = Token{
		enc: Encoding::Primitive,
		ty: Type::VisibleString,
		len: 0,
		header: &[0b0],
		body: "Unsupported characters: \x07\x00\x10".as_bytes(),
	};

	match String::from_token(&token) {
		Err(Unsupported) => return,
		Err(why) => panic!("Expected {:?}, got {:?}", Unsupported, why),
		Ok(_) => panic!("Invalid string accepted"),
	}
}

#[test]
fn printablestring() {
	let token = Token{
		enc: Encoding::Primitive,
		ty: Type::PrintableString,
		len: 0,
		header: &[0b0],
		body: "Western Cape".as_bytes(),
	};

	match String::from_token(&token) {
		Ok(String(s)) => assert_eq!(s, "Western Cape"),
		Err(why) => panic!("Could not parse String from Token: {:?}", why)
	}
}

#[test]
fn invalid_printablestring() {
	let token = Token{
		enc: Encoding::Primitive,
		ty: Type::PrintableString,
		len: 0,
		header: &[0b0],
		body: "Unsupported characters: *;<>@".as_bytes(),
	};

	match String::from_token(&token) {
		Err(Malformed) => return,
		Err(why) => panic!("Expected {:?}, got {:?}", Malformed, why),
		Ok(_) => panic!("Invalid string accepted"),
	}
}