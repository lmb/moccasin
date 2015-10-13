use std::{cmp, fmt};
use std::iter::Peekable;
use std::slice::Iter;

use {Encoding, Token};
use super::TypeError;
use super::TypeError::*;

const ARC_SHIFT: u8 = 1<<7;
const ARC_MASK:  u8 = (1<<7) - 1;

// TODO: Add const fn new
#[derive(Debug)]
pub struct StaticOid(pub &'static [u32]);

impl fmt::Display for StaticOid {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		try!(write!(f, "StaticOid("));

		for i in 0..self.0.len() {
			try!(write!(f, "{}", self.0[i]));

			if i != self.0.len() - 1 {
				try!(write!(f, "."));
			}
		}

		write!(f, ")")
	}
}

#[macro_export]
macro_rules! oid {
	( $( $x:expr ),* ) => [{
		StaticOid(&[$( $x, )*])
	}]
}

#[derive(Debug, PartialEq, Eq)]
pub struct Oid {
	arcs: [u32; 12],
	n: u8,
}

impl Oid {
	pub fn from_token(token: &Token) -> Result<Oid, TypeError> {
		if token.enc != Encoding::Primitive {
			return Err(Malformed);
		}

		if token.body.len() == 0 {
			return Err(Malformed)
		}

		let mut oid = Oid{arcs: [0u32; 12], n: 0};

		let mut iter: Peekable<Iter<u8>> = token.body.iter().peekable();

		// 8.19.4 + .5
		// If first arc is 2, values > 39 can be encoded for the second
		// one.
		{
			let arc = try!(Self::parse_arc(&mut iter));
			let first = cmp::min(arc, 80) / 40;

			try!(oid.append(first));

			let second = arc - (first * 40);
			try!(oid.append(second));
		}

		while let Some(_) = iter.peek() {
			let arc = try!(Self::parse_arc(&mut iter));
			try!(oid.append(arc));
		}

		Ok(oid)
	}

	fn parse_arc<'a>(iter: &mut Iterator<Item=&'a u8>) -> Result<u32, TypeError> {
		let mut arc = 0u32;

		for byte in iter {
			if arc == 0 && *byte == 0x80 {
				// 8.19.2 "the leading octet of the subidentifier shall not have the
				// value 0x80"
				return Err(Malformed)
			}

			arc = match arc.checked_mul(ARC_SHIFT as u32) {
				Some(v) => v | (*byte & ARC_MASK) as u32,
				None => return Err(Unsupported)
			};

			// 8.19.2 "[...] last in the series: bit 8 of the last octet is zero;"
			if *byte & 0x80 == 0 {
				return Ok(arc)
			}
		}

		return Err(Malformed)
	}

	fn append(&mut self, arc: u32) -> Result<(), TypeError> {
		if self.n as usize >= self.arcs.len() {
			return Err(Unsupported);
		}

		self.arcs[self.n as usize] = arc;

		if let Some(n) = self.n.checked_add(1) {
			self.n = n;
		} else {
			return Err(Unsupported);
		}

		Ok(())
	}
}

impl fmt::Display for Oid {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		try!(write!(f, "Oid("));

		for i in 0..self.n as usize {
			try!(write!(f, "{}", self.arcs[i]));

			if i != (self.n - 1) as usize {
				try!(write!(f, "."));
			}
		}

		write!(f, ")")
	}
}

impl PartialEq<StaticOid> for Oid {
	fn eq(&self, other: &StaticOid) -> bool {
		self.n as usize == other.0.len() &&
			self.arcs[0 .. self.n as usize] == *other.0
	}
}