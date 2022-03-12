use std::rc::Rc;
use std::cell::RefCell;
use self::List::{Cons, Nil};

pub fn main() {
	let value = Rc::new(RefCell::new(1));

	let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

	let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
	let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

	*value.borrow_mut() -= 1;
	if let Cons(v, _) = &b {
		*v.borrow_mut() -= 1;
	}

	println!("a = {:?}", a);
	println!("b = {:?}", b);
	println!("c = {:?}", c);
}

#[derive(Debug)]
enum List {
	Cons(Rc<RefCell<i32>>, Rc<List>),
	Nil
}