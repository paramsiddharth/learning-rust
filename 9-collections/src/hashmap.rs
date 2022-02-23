use std::collections::HashMap;

pub fn main() {
	let mut scores = HashMap::new();
	scores.insert(String::from("Param"), 20);
	println!("{:?}", scores);

	let users = vec![String::from("Param"), String::from("Kabir")];
	let points = vec![20, 25];
	let mut points: HashMap<_, _> = users.into_iter().zip(points.into_iter()).collect();
	points.insert("Lata".to_string(), 30);
	points.insert("Santosh".to_string(), 30);
	println!("{:?}", points);

	let text = String::from("Param");
	let mut freq = HashMap::new();
	for ch in text.chars() {
		*freq.entry(ch).or_insert(0) += 1;
	}
	println!("{:?}", freq);

	let alphabet = "Alpha Beta Gamma Delta Gamma Gamma Delta Pamma";
	let mut freq = HashMap::new();
	for word in alphabet.split_whitespace() {
		let x = freq.entry(word).or_insert(0);
		*x += 1;
	}
	println!("{:?}", freq);
}