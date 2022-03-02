pub struct Counter {
	pub count_: u32
}

impl Counter {
	pub fn new() -> Counter {
		Counter { count_: 0 }
	}
}

impl Iterator for Counter {
	type Item = u32;

	fn next(&mut self) -> Option<Self::Item> {
		if self.count_ < 5 {
			self.count_ += 1;
			Some(self.count_)
		} else {
			None
		}
	}
}