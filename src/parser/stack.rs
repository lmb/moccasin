pub struct FixedStack {
	data: [usize; 10],
	i: Option<u8>
}

impl FixedStack {
	pub fn new() -> FixedStack {
		FixedStack{
			data: [0; 10],
			i: None
		}
	}

	pub fn depth(&self) -> u8 {
		match self.i {
			Some(n) => n + 1,
			None => 0
		}
	}

	pub fn peek(&self) -> Option<usize> {
		match self.i {
			Some(i) => Some(self.data[i as usize]),
			_ => None
		}
	}

	pub fn discard(&mut self) {
		if let Some(i) = self.i {
			self.i = i.checked_sub(1)
		}
	}

	pub fn push(&mut self, val: usize) -> Result<(), ()> {
		let i = match self.i {
			None => Some(0),
			Some(i) => i.checked_add(1)
		};

		if let Some(i) = i {
			if i as usize >= self.data.len() {
				return Err(())
			}

			self.i = Some(i);
			self.data[i as usize] = val;
			return Ok(())
		}

		Err(())
	}
}
