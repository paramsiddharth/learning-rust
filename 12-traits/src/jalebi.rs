use crate::edible::Edible;

pub struct Jalebi {
	pub sugar: f32
}

impl Jalebi {
	pub fn new(sugar: f32) -> Jalebi {
		Jalebi { sugar }
	}
}

impl Edible for Jalebi {
	fn eat(&self) {
		println!("Sweet!");
	}
}