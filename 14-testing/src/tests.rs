use super::*;

#[test]
fn exploration() {
	assert_eq!(2 + 2, 4);
}

#[test]
#[should_panic]
fn another() {
	panic!("This must fail.");
}

#[test]
fn larger_can_hold_smaller() {
	let larger = Rectangle {
		width: 3,
		height: 4
	};
	let smaller = Rectangle {
		width: 3,
		height: 2
	};

	assert!(
		larger.can_hold(&smaller),
		"Larger: {}, Smaller: {}",
		larger,
		smaller
	);
}

#[test]
fn smaller_cannot_hold_larger() {
	let larger = Rectangle {
		width: 3,
		height: 4
	};
	let smaller = Rectangle {
		width: 3,
		height: 2
	};

	assert!(
		!smaller.can_hold(&larger),
		"Larger: {}, Smaller: {}",
		larger,
		smaller
	);
}

#[test]
#[should_panic(expected = "Can't do nothing")]
#[ignore]
fn pain_in_life() {
	do_something(String::from("Nothing"));
}

#[test]
fn देखते_हैं() -> Result<(), String> {
	if 1 + 1 == 2 {
		Ok(())
	} else {
		Err(String::from("This is not OK!"))
	}
}