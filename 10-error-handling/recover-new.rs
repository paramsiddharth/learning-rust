use std::fs::File;
use std::io::ErrorKind;

fn main() {
	let filename = "Hi.txt";
	let _f = File::open(filename).unwrap_or_else(|error| {
		if error.kind() == ErrorKind::NotFound {
			File::create(filename).unwrap_or_else(|error| {
				panic!("Failed to create the file. '{}'!", error);
			})
		} else {
			panic!("Failed to open the file. '{}'!", error);
		}
	});
}