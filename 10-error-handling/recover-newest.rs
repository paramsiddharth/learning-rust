use std::fs::File;

fn main() {
	let filename = "Hi.txt";
	let _f = File::open(filename).expect(format!("Failed to open '{}'!", filename).as_str());
}