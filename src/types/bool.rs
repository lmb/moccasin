use {Token, Tag, Error};
use types::TokenType;
use Error::MalformedToken;
use Encoding;

#[derive(Debug)]
pub struct Bool(pub bool);

impl<'a> TokenType<'a> for Bool {
	fn matches(tag: Tag) -> bool {
		tag == Tag::Bool
	}

	fn encoding() -> Encoding {
		Encoding::Primitive
	}

	fn from_token(token: &Token) -> Result<Bool, Error> {
		if token.body.len() != 1 {
			return Err(MalformedToken);
		}

		match token.body[0] {
			0x00 => Ok(Bool(false)),
			0xFF => Ok(Bool(true)),
			_    => Err(MalformedToken)
		}
	}
}
