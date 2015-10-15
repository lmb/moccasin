pub struct Iter<'a> {
	slice: &'a [u8],
	i: usize,
}

impl<'a> Iter<'a> {
	pub fn new(bytes: &'a [u8]) -> Iter<'a> {
		Iter {
			slice: bytes,
			i: 0
		}
	}

	pub fn peek(&self) -> Option<&'a u8> {
		self.slice.get(self.i + 1)
	}

	pub fn pos(&self) -> usize {
		self.i
	}

	pub fn subslice(&self, start: usize, end: usize) -> &'a [u8] {
		&self.slice[start..end]
	}
}

impl<'a> Iterator for Iter<'a> {
	type Item = &'a u8;

	fn next(&mut self) -> Option<&'a u8> {
		let result = self.slice.get(self.i);
		self.i += 1;
		result
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let remaining = self.slice.len() - self.i;
		(remaining, Some(remaining))
	}
}

impl<'a> ExactSizeIterator for Iter<'a> {
	fn len(&self) -> usize {
		self.slice.len()
	}
}

#[cfg(test)]
mod tests {
	use super::Iter;

	#[test]
	fn check_size_hint() {
		static DATA: &'static [u8] = &[1,2,3];

		let mut it = Iter::new(DATA);

		assert_eq!(it.pos(), 0);
		assert_eq!(it.len(), DATA.len());
		assert_eq!(it.size_hint(), (DATA.len(), Some(DATA.len())));

		it.next().unwrap();

		assert_eq!(it.pos(), 1);
		assert_eq!(it.len(), DATA.len());
		assert_eq!(it.size_hint(), (DATA.len()-1, Some(DATA.len()-1)));
	}
}
