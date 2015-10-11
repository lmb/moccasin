pub use self::string::String;
pub use self::oid::{Oid, StaticOid};

mod string;
#[macro_use]
mod oid;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TypeError {
	Unsupported,
	TypeMismatch,
	Malformed,
}

#[cfg(test)]
mod tests;
