use std::fs::File;
use std::io::{self, Read};

fn main() {
	let filename = "Hi.txt";
	let _f = get_file(filename).expect(
		format!("Failed to open '{}'!", filename).as_str()
	);
}

fn get_file(filename: &str) -> Result<String, io::Error> {
	let f = File::open(filename);

	let mut f = match f {
		Ok(file) => file,
		Err(e) => return Err(e)
	};

	let mut s = String::new();

	match f.read_to_string(&mut s) {
		Ok(_) => Ok(s),
		Err(e) => Err(e)
	}
}