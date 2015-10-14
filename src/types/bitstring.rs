use std::mem::size_of;
use std::ops::{Shl, Shr, BitOr};
use num::traits::{Unsigned, NumCast, cast};

use {Token, Error};
use types::FromToken;
use Error::{MalformedToken, OutOfMemory};
use Encoding::Primitive;

#[derive(Debug)]
pub struct Bitstring<'a>(&'a[u8], u8);

impl<'a> FromToken<'a> for Bitstring<'a> {
	fn from_token(token: &Token<'a>) -> Result<Bitstring<'a>, Error> {
		// 8.6.2.2 and 10.2
		// At least one byte of body (which specifies unused bits in last byte)
		// and of primitive encoding.
		if token.body.len() < 1 || token.enc != Primitive {
			return Err(MalformedToken);
		}

		let unused = token.body[0];

		// 8.6.2.2
		// DER mandates minimal encoding and with unused > 7 we could have
		// omitted at least one byte.
		if unused > 7 {
			return Err(MalformedToken);
		}

		// 8.6.2.3
		// An empty bitstring is of length 1 (for unused byte) with no unused
		// bits.
		if token.body.len() == 1 {
			return match unused {
				0 => Ok(Bitstring(&[], 0)),
				_ => Err(MalformedToken)
			}
		}

		// 11.2.1
		// Unused bits must be set to 0
		if unused > 0 {
			let mask = (1 << unused) - 1;

			if token.body[token.body.len()-1] & mask != 0 {
				return Err(MalformedToken);
			}
		}

		// 11.2.2
		// Trailing 0 bits should be omitted
		if token.body[token.body.len()-1] == 0 {
			return Err(MalformedToken);
		}

		Ok(Bitstring(&token.body[1..], unused))
	}
}

impl<'a> Bitstring<'a> {
	pub fn as_unsigned<T>(&self) -> Result<T, Error>
		where T: Copy + Unsigned + NumCast + Shl<u8, Output = T> + Shr<u8, Output=T> + BitOr<Output = T>
	{
		if self.0.len() > size_of::<T>() {
			return Err(OutOfMemory);
		}

		let mut result = T::zero();

		for byte in self.0 {
			result = (result << 8u8) | cast(*byte).unwrap();
		}

		Ok(result >> self.1)
	}
}
