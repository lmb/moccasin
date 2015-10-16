extern crate moccasin;
extern crate rustc_serialize;

use std::env;

use rustc_serialize::hex::FromHex;
use moccasin::prelude::*;

#[cfg(not(test))]
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