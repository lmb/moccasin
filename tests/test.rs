extern crate moccasin;

use moccasin::Encoding::*;
use moccasin::Tag::*;
use moccasin::Class::*;
use moccasin::Parser;
use moccasin::Token;

#[test]
fn x509() {
	let data = include_bytes!("cert.der");

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