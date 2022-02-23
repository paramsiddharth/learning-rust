use crate::edible::Edible;

pub struct Samosa {
	pub masala: i32
}

impl Samosa {
	pub fn new(masala: i32) -> Samosa {
		Samosa { masala }
	}
}

impl Edible for Samosa {
	fn eat(&self) {
		println!("Tasty!");
	}
}