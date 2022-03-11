use std::cell::RefCell;
use std::mem::drop;

pub fn main() {
	let x = RefCell::new(1);
	let mut y = x.borrow_mut();
	*y -= 1;
	drop(y);
	println!("{:?}", x.borrow());
}