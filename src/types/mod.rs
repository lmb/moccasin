pub use self::string::String;
pub use self::oid::{Oid, StaticOid};
pub use self::int::Int;

mod string;
#[macro_use]
mod oid;
mod int;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TypeError {
	Unsupported,
	TypeMismatch,
	Malformed,
	OutOfMemory,
}

#[cfg(test)]
mod tests;
