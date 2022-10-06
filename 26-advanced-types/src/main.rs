type YesNoMaybe = Option<bool>;


fn tell_me(num: i32) -> YesNoMaybe {
	match num {
		1 => Some(true),
		0 => Some(false),
		_ => None
	}
}

fn main() {
	let num = -1;

	match tell_me(num) {
		Some(b) => {
			if b {
				println!("Yes!");
			} else {
				println!("No!");
			}
		},
		None => {
			println!("Maybe!");
		}
	}

	match String::from("5").trim().parse::<i32>() {
		Ok(_) => { 5; },
		Err(_) => { "Failure"; }
	};
}
