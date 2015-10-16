use std::mem::size_of;
use std::ops::{Shl, BitXor, BitOr};
use num::traits::{Signed, NumCast, cast};

use {Token, Tag, Encoding, Error};
use types::TokenType;
use Error::*;

#[derive(Debug)]
pub struct Int<T>(pub T)
	// Trait bound from hell
	where T: Copy + Signed + NumCast + Shl<u8, Output = T> + BitXor<Output = T> + BitOr<Output = T>;

impl<'a, T> TokenType<'a> for Int<T>
	where T: Copy + Signed + NumCast + Shl<u8, Output = T> + BitXor<Output = T> + BitOr<Output = T>
{
	fn matches(tag: Tag) -> bool {
		tag == Tag::Int
	}

	fn encoding() -> Encoding {
		Encoding::Primitive
	}

	fn from_token(token: &Token) -> Result<Int<T>, Error> {
		if token.body.len() == 0 {
			return Err(MalformedToken);
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
				return Err(MalformedToken)
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

