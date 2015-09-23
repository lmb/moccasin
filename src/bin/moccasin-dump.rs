extern crate moccasin;

use std::fs::File;
use std::io::prelude::*;

use moccasin::Parser;

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
			Ok(t) => {
				println!("{:?}", t);
			},
			Err(why) => panic!("Parse error: {:?}", why)
		}
	}
}