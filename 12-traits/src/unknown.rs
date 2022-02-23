use crate::edible::Edible;

pub struct Unknown {}

impl Unknown {
	pub fn new() -> Unknown {
		Unknown {}
	}
}

impl Edible for Unknown {}