use {Parser, Token, Tag, Error};
use types::TokenType;
use Encoding;

#[derive(Debug)]
pub struct Octetstring<'a>(pub &'a [u8]);

impl<'a> Octetstring<'a> {
	pub fn parser(&self) -> Parser<'a> {
		Parser::new(self.0)
	}
}

impl<'a> TokenType<'a> for Octetstring<'a> {
	fn matches(tag: Tag) -> bool {
		tag == Tag::Octetstring
	}

	fn encoding() -> Encoding {
		Encoding::Primitive
	}

	fn from_token(token: &Token<'a>) -> Result<Self, Error> {
		Ok(Octetstring(token.body))
	}
}