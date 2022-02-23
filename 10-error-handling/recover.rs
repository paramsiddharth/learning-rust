use std::fs::File;
use std::io::ErrorKind;

fn main() {
	let filename = "Hi.txt";
	let _f = File::open(filename);

	let _f = match _f {
		Ok(file) => file,
		Err(error) => match error.kind() {
			ErrorKind::NotFound => match File::create(filename) {
				Ok(fc) => fc,
				Err(e) => panic!("Failed to create the file. '{:?}'!", e)
			},
			e => panic!("Failed to open the file. '{:?}'!", e)
		}
	};
}