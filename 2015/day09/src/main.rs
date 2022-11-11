use std::{fs, collections::HashMap};

fn main() {
	let input = fs::read_to_string("input").unwrap();

	let mut distances: HashMap<(String, String), u32> = HashMap::new();

	for line in input.lines() {
		let mut tokens = line.split(" = ");
		let mut links = tokens.next().unwrap().split(" to ");
		let link1 = links.next().unwrap().to_string();
		let link2 = links.next().unwrap().to_string();
		let dist = tokens.next().unwrap().parse().unwrap();

		println!("{} {} {}", link1, link2, dist);

		distances.insert((link1, link2), dist);
	}

	println!("Part1: {}", 0);
	println!("Part2: {}", 0);
}
