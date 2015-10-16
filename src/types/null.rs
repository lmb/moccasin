use {Token, Tag, Error};
use types::TokenType;
use Error::MalformedToken;
use Encoding;

#[derive(Debug)]
pub struct Null;

impl<'a> TokenType<'a> for Null {
	fn matches(tag: Tag) -> bool {
		tag == Tag::Null
	}

	fn encoding() -> Encoding {
		Encoding::Primitive
	}

	fn from_token(token: &Token<'a>) -> Result<Null, Error> {
		if token.body.len() != 0 {
			return Err(MalformedToken);
		}

		Ok(Null)
	}
}
