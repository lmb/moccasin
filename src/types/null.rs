use {Token, Error};
use types::FromToken;
use Error::MalformedToken;
use Encoding::Primitive;

#[derive(Debug)]
pub struct Null;

impl<'a> FromToken<'a> for Null {
	fn from_token(token: &Token<'a>) -> Result<Null, Error> {
		if token.body.len() != 0 || token.enc != Primitive {
			return Err(MalformedToken);
		}

		Ok(Null)
	}
}
