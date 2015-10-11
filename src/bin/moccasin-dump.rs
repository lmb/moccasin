extern crate moccasin;

use std::fs::File;
use std::io::prelude::*;

use moccasin::{Parser, Token};
use moccasin::types::{String};
use moccasin::Type::PrintableString;

// For some reason, cargo test builds this executable and complains about unused
// code.
#[allow(dead_code)]
fn main() {
	let mut data: Vec<u8> = Vec::new();

	let mut file = match File::open("tests/cert.der") {
		Err(why) => panic!("Can't open file: {:?}", why),
		Ok(file) => file,
	};

	if let Err(why) = file.read_to_end(&mut data) {
		panic!("Can't read file: {:?}", why)
	}

	let p = Parser::new(&data);

	for token in p {
		match token {
			Ok(ref t @ Token{ty: PrintableString, ..}) => {
				let s = if let Ok(String(string)) = String::from_token(&t) {
					string
				} else {
					"INVALID"
				};

				let &Token{ref ty, ref enc, ref body, ref depth, ..} = t;

				println!("d={: >2} {:?}, {:?}, {: >3}: {}", depth, enc, ty, body.len(), s);
			},
			Ok(Token{enc, ty, body, depth, ..}) => {
				println!("d={: >2} {:?}, {:?}, {: >3}", depth, enc, ty, body.len())
			},
			Err(why) => panic!("Parse error: {:?}", why)
		}
	}
}