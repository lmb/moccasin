use std::mem::size_of;
use std::ops::{Shl, BitXor, BitOr};
use num::traits::{Signed, NumCast, cast};

use {Token, Encoding};
use super::TypeError;
use super::TypeError::*;

#[derive(Debug)]
pub struct Int<T>(pub T)
	// Trait bound from hell
	where T: Copy + Signed + NumCast + Shl<u8, Output = T> + BitXor<Output = T> + BitOr<Output = T>;

impl<T> Int<T>
	where T: Copy + Signed + NumCast + Shl<u8, Output = T> + BitXor<Output = T> + BitOr<Output = T>
{
	pub fn from_token(token: &Token) -> Result<Int<T>, TypeError> {
		if token.enc != Encoding::Primitive {
			return Err(Malformed);
		}

		if token.body.len() == 0 {
			return Err(Malformed);
		}

		if token.body.len() > size_of::<T>() {
			return Err(OutOfMemory);
		}

		if token.body.len() > 1 {
			// 8.3.2
			// Ensure ints are encoded as short as possible
			let leading = {
				((token.body[0] as u16) << 1) | ((token.body[1] as u16) >> 7)
			};

			if leading == 0b0000_0000_0 || leading == 0b1111_1111_1 {
				return Err(Malformed)
			}
		}

		let mut result = T::zero();

		for byte in token.body {
			result = (result << 8u8) | cast(*byte).unwrap();
		}

		// Sign extend
		let mask = T::one() << cast(token.body.len() * 8 - 1).unwrap();
		Ok(Int((result ^ mask) - mask))
	}
}

