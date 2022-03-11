mod boxes;
#[path="./my-box.rs"]
mod my_box;

use std::mem::drop;
use my_box::MyBox;

fn main() {
	boxes::main();

	let x = 2;
	let y = MyBox::new(x);

	assert_eq!(2, *y);
	println!("Attempting to drop object.");
	drop(y);
	println!("Object dropped.");
	assert_eq!(2, x);
}