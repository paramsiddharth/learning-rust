use std::fs::File;

fn main() {
	let filename = "Hi.txt";
	let _f = File::open(filename).unwrap();
}