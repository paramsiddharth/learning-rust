mod boxes;
#[path="./my-box.rs"]
mod my_box;
mod rc;
mod refcell;
#[path="./rc-refcell.rs"]
mod rc_refcell;
#[path="./ref-cycle.rs"]
mod ref_cycle;
mod weak;

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

	rc::main();
	refcell::main();
	rc_refcell::main();
	ref_cycle::main();
	weak::main();
}