use std::fs::File;
use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
	let filename = "Hi.txt";
	let mut s = String::new();
	let _s = File::open(filename)?.read_to_string(&mut s)?;
	Ok(())
}