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
		(Constructed, Sequence, 902),
		(Constructed, Sequence, 751),
		(Constructed, Composed(Context, 0), 3),
		(Primitive,   Int, 1),
		(Primitive,   Int, 16),
		(Constructed, Sequence, 13),
		(Primitive,   Oid, 9),
		(Primitive,   Null, 0),
		(Constructed, Sequence, 206),
		(Constructed, Set, 11),
		(Constructed, Sequence, 9),
		(Primitive,   Oid, 3),
		(Primitive,   PrintableString, 2),
		(Constructed, Set, 21),
		(Constructed, Sequence, 19),
		(Primitive,   Oid, 3),
		(Primitive,   PrintableString, 12),
		(Constructed, Set, 18),
		(Constructed, Sequence, 16),
		(Primitive,   Oid, 3),
		(Primitive,   PrintableString, 9),
		(Constructed, Set, 29),
		(Constructed, Sequence, 27),
		(Primitive,   Oid, 3),
		(Primitive,   PrintableString, 20),
		(Constructed, Set, 40),
		(Constructed, Sequence, 38),
		(Primitive,   Oid, 3),
		(Primitive,   PrintableString, 31),
		(Constructed, Set, 33),
		(Constructed, Sequence, 31),
		(Primitive,   Oid, 3),
		(Primitive,   PrintableString, 24),
		(Constructed, Set, 40),
		(Constructed, Sequence, 38),
		(Primitive,   Oid, 9),
		(Primitive,   Ia5String, 25),
		(Constructed, Sequence, 30),
		(Primitive,   UtcTime, 13),
		(Primitive,   UtcTime, 13),
		(Constructed, Sequence, 102),
		(Constructed, Set, 11),
		(Constructed, Sequence, 9),
		(Primitive,   Oid, 3),
		(Primitive,   PrintableString, 2),
		(Constructed, Set, 19),
		(Constructed, Sequence, 17),
		(Primitive,   Oid, 3),
		(Primitive,   PrintableString, 10),
		(Constructed, Set, 22),
		(Constructed, Sequence, 20),
		(Primitive,   Oid, 3),
		(Primitive,   PrintableString, 13),
		(Constructed, Set, 19),
		(Constructed, Sequence, 17),
		(Primitive,   Oid, 3),
		(Primitive,   PrintableString, 10),
		(Constructed, Set, 21),
		(Constructed, Sequence, 19),
		(Primitive,   Oid, 3),
		(Primitive,   Utf8String, 12),
		(Constructed, Sequence, 159),
		(Constructed, Sequence, 13),
		(Primitive,   Oid, 9),
		(Primitive,   Null, 0),
		(Primitive,   Bitstring, 141),
		(Constructed, Composed(Context, 3), 203),
		(Constructed, Sequence, 200),
		(Constructed, Sequence, 29),
		(Primitive,   Oid, 3),
		(Primitive,   Octetstring, 22),
		(Constructed, Sequence, 64),
		(Primitive,   Oid, 3),
		(Primitive,   Octetstring, 57),
		(Constructed, Sequence, 50),
		(Primitive,   Oid, 8),
		(Primitive,   Octetstring, 38),
		(Constructed, Sequence, 35),
		(Primitive,   Oid, 3),
		(Primitive,   Octetstring, 28),
		(Constructed, Sequence, 12),
		(Primitive,   Oid, 3),
		(Primitive,   Bool, 1),
		(Primitive,   Octetstring, 2),
		(Constructed, Sequence, 13),
		(Primitive,   Oid, 9),
		(Primitive,   Null, 0),
		(Primitive,   Bitstring, 129)
	];

	let p = Parser::new(&data);

	for (token, expected) in p.zip(tokens.iter()) {
		match token {
			Ok(Token{enc, ty, len, body, ..}) => {
				assert_eq!(enc,        expected.0);
				assert_eq!(ty,         expected.1);
				assert_eq!(len,        expected.2);
				assert_eq!(body.len(), expected.2);
			},
			Err(e) => panic!("Expected {:?}, got {:?}", expected, e)
		}
	}
}