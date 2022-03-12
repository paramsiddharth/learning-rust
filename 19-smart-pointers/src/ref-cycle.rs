use std::rc::Rc;
use std::cell::RefCell;
use self::List::{Cons, Nil};

pub fn main() {
	let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

	println!("Count (a) = {}", Rc::strong_count(&a));
	println!("Next (a) = {:?}", a.tail());

	let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

	println!("b created.");
	println!("Count (b) = {}", Rc::strong_count(&b));
	println!("Next (b) = {:?}", b.tail());

	if let Some(link) = a.tail() {
		*link.borrow_mut() = Rc::clone(&b);
	}

	println!("Link created.");
	println!("Count (b) = {}", Rc::strong_count(&b));
	println!("Count (a) = {}", Rc::strong_count(&a));

	// Error rendering
	// println!("Next (a) = {:?}", a.tail());
}

#[derive(Debug)]
enum List {
	Cons(i32, RefCell<Rc<List>>),
	Nil
}

impl List {
	fn tail(&self) -> Option<&RefCell<Rc<List>>> {
		match self {
			Cons(_, item) => Some(item),
			Nil => None
		}
	}
}