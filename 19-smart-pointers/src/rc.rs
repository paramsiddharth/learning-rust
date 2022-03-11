use std::rc::Rc;
use self::List::{Cons, Nil};

pub fn main() {
	/*
	 * 
	 *     3
	 *      \
	 *       5 - 10 - Nil
	 *      /
	 *     4
	 *	
	 */
	let a = Cons(10, Rc::new(Nil));
	let a = Cons(5, Rc::new(a));
	let a = Rc::new(a);
	println!("Count (a) = {}", Rc::strong_count(&a));
	let _b = Cons(3, Rc::clone(&a));
	println!("Count (a) = {}", Rc::strong_count(&a));
	{
		let _c = Cons(4, Rc::clone(&a));
		println!("Count (a) = {}", Rc::strong_count(&a));
	}
	println!("Count (a) = {}", Rc::strong_count(&a));
}

enum List {
	Cons(i32, Rc<List>),
	Nil
}