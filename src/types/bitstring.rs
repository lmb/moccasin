use {Token, Tag, Error};
use types::TokenType;
use Error::MalformedToken;
use Encoding;

#[derive(Debug)]
pub struct Bitstring<'a>(&'a[u8], u8);

impl<'a> TokenType<'a> for Bitstring<'a> {
	fn matches(tag: Tag) -> bool {
		tag == Tag::Bitstring
	}

	fn encoding() -> Encoding {
		Encoding::Primitive
	}

	fn from_token(token: &Token<'a>) -> Result<Bitstring<'a>, Error> {
		// 8.6.2.2 and 10.2
		// At least one byte of body (which specifies unused bits in last byte)
		// and of primitive encoding.
		if token.body.len() < 1 {
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
	pub fn len_bits(&self) -> usize {
		self.0.len() * 8 - (self.1 as usize)
	}

	pub fn is_set(&self, pos: usize) -> bool {
		// According to X.680 22.7 & X.690 11.2.2 trailing 0 will be omitted.
		// Treat them as default false.
		if pos >= self.len_bits() {
			return false;
		}

		let i = pos / 8;
		let mask = 1 << (7 - ((pos % 8) as u8));

		self.0[i] & mask != 0
	}
}
