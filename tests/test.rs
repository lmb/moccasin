extern crate moccasin;

use std::fs::File;
use std::io::prelude::*;

use moccasin::Encoding::*;
use moccasin::Type::*;
use moccasin::Class::*;
use moccasin::Parser;
use moccasin::Token;

#[test]
fn x509() {
	let mut data: Vec<u8> = Vec::new();

	let mut file = match File::open("tests/cert.der") {
		Err(why) => panic!("Can't open file: {:?}", why),
		Ok(file) => file,
	};

	if let Err(why) = file.read_to_end(&mut data) {
		panic!("Can't read cert.der: {:?}", why)
	}

	let tokens = [
		Token(Constructed, Sequence, 902),
		Token(Constructed, Sequence, 751),
		Token(Constructed, Composed(Context, 0), 3),
		Token(Primitive,   Int, 1),
		Token(Primitive,   Int, 16),
		Token(Constructed, Sequence, 13),
		Token(Primitive,   Oid, 9),
		Token(Primitive,   Null, 0),
		Token(Constructed, Sequence, 206),
		Token(Constructed, Set, 11),
		Token(Constructed, Sequence, 9),
		Token(Primitive,   Oid, 3),
		Token(Primitive,   PrintableString, 2),
		Token(Constructed, Set, 21),
		Token(Constructed, Sequence, 19),
		Token(Primitive,   Oid, 3),
		Token(Primitive,   PrintableString, 12),
		Token(Constructed, Set, 18),
		Token(Constructed, Sequence, 16),
		Token(Primitive,   Oid, 3),
		Token(Primitive,   PrintableString, 9),
		Token(Constructed, Set, 29),
		Token(Constructed, Sequence, 27),
		Token(Primitive,   Oid, 3),
		Token(Primitive,   PrintableString, 20),
		Token(Constructed, Set, 40),
		Token(Constructed, Sequence, 38),
		Token(Primitive,   Oid, 3),
		Token(Primitive,   PrintableString, 31),
		Token(Constructed, Set, 33),
		Token(Constructed, Sequence, 31),
		Token(Primitive,   Oid, 3),
		Token(Primitive,   PrintableString, 24),
		Token(Constructed, Set, 40),
		Token(Constructed, Sequence, 38),
		Token(Primitive,   Oid, 9),
		Token(Primitive,   Ia5String, 25),
		Token(Constructed, Sequence, 30),
		Token(Primitive,   UtcTime, 13),
		Token(Primitive,   UtcTime, 13),
		Token(Constructed, Sequence, 102),
		Token(Constructed, Set, 11),
		Token(Constructed, Sequence, 9),
		Token(Primitive,   Oid, 3),
		Token(Primitive,   PrintableString, 2),
		Token(Constructed, Set, 19),
		Token(Constructed, Sequence, 17),
		Token(Primitive,   Oid, 3),
		Token(Primitive,   PrintableString, 10),
		Token(Constructed, Set, 22),
		Token(Constructed, Sequence, 20),
		Token(Primitive,   Oid, 3),
		Token(Primitive,   PrintableString, 13),
		Token(Constructed, Set, 19),
		Token(Constructed, Sequence, 17),
		Token(Primitive,   Oid, 3),
		Token(Primitive,   PrintableString, 10),
		Token(Constructed, Set, 21),
		Token(Constructed, Sequence, 19),
		Token(Primitive,   Oid, 3),
		Token(Primitive,   Utf8String, 12),
		Token(Constructed, Sequence, 159),
		Token(Constructed, Sequence, 13),
		Token(Primitive,   Oid, 9),
		Token(Primitive,   Null, 0),
		Token(Primitive,   Bitstring, 141),
		Token(Constructed, Composed(Context, 3), 203),
		Token(Constructed, Sequence, 200),
		Token(Constructed, Sequence, 29),
		Token(Primitive,   Oid, 3),
		Token(Primitive,   Octetstring, 22),
		Token(Constructed, Sequence, 64),
		Token(Primitive,   Oid, 3),
		Token(Primitive,   Octetstring, 57),
		Token(Constructed, Sequence, 50),
		Token(Primitive,   Oid, 8),
		Token(Primitive,   Octetstring, 38),
		Token(Constructed, Sequence, 35),
		Token(Primitive,   Oid, 3),
		Token(Primitive,   Octetstring, 28),
		Token(Constructed, Sequence, 12),
		Token(Primitive,   Oid, 3),
		Token(Primitive,   Bool, 1),
		Token(Primitive,   Octetstring, 2),
		Token(Constructed, Sequence, 13),
		Token(Primitive,   Oid, 9),
		Token(Primitive,   Null, 0),
		Token(Primitive,   Bitstring, 129)
	];

	let p = Parser::new(&data);

	for (token, expected) in p.zip(tokens.iter()) {
		match token {
			Ok(token) => assert_eq!(token, *expected),
			Err(e) => panic!("Expected {:?}, got {:?}", expected, e)
		}
	}
}