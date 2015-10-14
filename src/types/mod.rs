pub use self::string::String;
pub use self::oid::{Oid, ConstOid};
pub use self::int::Int;
pub use self::bool::Bool;
pub use self::null::Null;
pub use self::bitstring::Bitstring;
pub use self::time::Time;

mod string;
#[macro_use]
mod oid;
mod int;
mod bool;
mod null;
mod bitstring;
mod time;

use {Token, Error};

pub trait FromToken<'a> {
	fn from_token(token: &Token<'a>) -> Result<Self, Error>;
}

#[cfg(test)]
mod tests;
