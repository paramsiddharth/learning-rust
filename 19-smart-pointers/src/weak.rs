use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub fn main() {
	let leaf = Rc::new(Node {
		value: 3,
		parent: RefCell::new(Weak::new()),
		// parent: RefCell::new(Rc::downgrade(&leaf)),
		children: RefCell::new(vec![])
	});

	println!("Parent (Leaf) = {:?}", leaf.parent.borrow().upgrade());
	println!("Strong count (leaf) = {}", Rc::strong_count(&leaf));
	println!("  Weak count (leaf) = {}", Rc::weak_count(&leaf));

	{
		let branch = Rc::new(Node {
			value: 5,
			parent: RefCell::new(Weak::new()),
			children: RefCell::new(vec![Rc::clone(&leaf)])
		});
	
		*leaf.parent.borrow_mut() = Rc::downgrade(&branch);

		println!("Strong count (branch) = {}", Rc::strong_count(&branch));
		println!("  Weak count (branch) = {}", Rc::weak_count(&branch));
		
		println!("Parent (Leaf) = {:?}", leaf.parent.borrow().upgrade());
		println!("Strong count (leaf) = {}", Rc::strong_count(&leaf));
		println!("  Weak count (leaf) = {}", Rc::weak_count(&leaf));
	}

	println!("Parent (Leaf) = {:?}", leaf.parent.borrow().upgrade());
	println!("Strong count (leaf) = {}", Rc::strong_count(&leaf));
	println!("  Weak count (leaf) = {}", Rc::weak_count(&leaf));
}

#[derive(Debug)]
#[allow(dead_code)]
struct Node {
	value: i32,
	parent: RefCell<Weak<Node>>,
	children: RefCell<Vec<Rc<Node>>>
}