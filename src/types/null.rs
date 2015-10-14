use {Token, Error};
use Error::MalformedToken;
use Encoding::Primitive;

#[derive(Debug)]
pub struct Null;

impl Null {
	pub fn from_token(token: &Token) -> Result<Null, Error> {
		if token.body.len() != 0 || token.enc != Primitive {
			return Err(MalformedToken);
		}

		Ok(Null)
	}
}
