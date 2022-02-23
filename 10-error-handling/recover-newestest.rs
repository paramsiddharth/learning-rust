use std::fs::File;
use std::io::{self, Read};

fn main() {
	let filename = "Hi.txt";
	let _f = get_file(filename).expect(
		format!("Failed to open '{}'!", filename).as_str()
	);
}

fn get_file(filename: &str) -> Result<String, io::Error> {
	let mut s = String::new();
	/* let mut f = File::open(filename)?;
	f.read_to_string(&mut s)?; */
	File::open(filename)?.read_to_string(&mut s)?;
	Ok(s)
}