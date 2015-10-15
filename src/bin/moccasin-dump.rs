extern crate moccasin;
extern crate rustc_serialize;

use std::env;

use rustc_serialize::hex::FromHex;
use moccasin::prelude::*;

// For some reason, cargo test builds this executable and complains about unused
// code.
#[allow(dead_code)]
fn main() {
	let mut args = env::args();
	args.next().unwrap();

	for arg in args {
		let bytes = arg.from_hex().unwrap();

		let p = Parser::new(&bytes);

		for t in p {
			println!("{:?}", t);
		}
	}
}