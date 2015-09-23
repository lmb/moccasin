pub use self::string::String;

mod string;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TypeError {
	Unsupported,
	TypeMismatch,
	Malformed,
}

#[cfg(test)]
mod tests;
