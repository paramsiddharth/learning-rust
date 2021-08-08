#![allow(unused)]

fn main() {
	let mut spaces = "       ";
	// spaces = spaces.len(); // This fails
	let spaces = spaces.len(); // (?) This doesn't, but a warning is shown, saying "spaces" doesn't need to be mutable
}