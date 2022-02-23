use crate::edible::Edible;

pub struct Chutney {
	pub flavour: String
}

impl Chutney {
	pub fn new(flavour: String) -> Chutney {
		Chutney { flavour }
	}
}

impl Edible for Chutney {
	fn eat(&self) {
		println!("Chatpati!");
	}
}