use {Token, Error};
use Error::MalformedToken;
use Encoding::Primitive;

#[derive(Debug)]
pub struct Bool(pub bool);

impl Bool {
	pub fn from_token(token: &Token) -> Result<Bool, Error> {
		if token.body.len() != 1 || token.enc != Primitive {
			return Err(MalformedToken);
		}

		match token.body[0] {
			0x00 => Ok(Bool(false)),
			0xFF => Ok(Bool(true)),
			_    => Err(MalformedToken)
		}
	}
}
