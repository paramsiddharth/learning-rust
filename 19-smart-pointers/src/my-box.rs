use std::ops::Deref;
use std::fmt::Debug;

#[derive(Debug)]
pub struct MyBox<T: Debug>(T);

impl<T: Debug> MyBox<T> {
	pub fn new(x: T) -> MyBox<T> {
		MyBox(x)
	}
}

impl<T: Debug> Deref for MyBox<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<T: Debug> Drop for MyBox<T> {
	fn drop(&mut self) {
		println!("Dropping {:?}...", &self.0);
	}
}