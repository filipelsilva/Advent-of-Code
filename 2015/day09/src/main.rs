use std::{fs, collections::HashMap};

#[derive(Eq, Hash, PartialEq)]
struct Node {
	id: String,
	links: Vec<Node>,
}

fn main() {
	let input = fs::read_to_string("input").unwrap();

	let mut nodes: HashMap<String, Node> = HashMap::new();

	for line in input.lines() {
		let mut tokens = line.split(" = ");
		let mut links = tokens.next().unwrap().split(" to ");
		let link1 = links.next().unwrap().to_string();
		let link2 = links.next().unwrap().to_string();
		let dist: usize = tokens.next().unwrap().parse().unwrap();

		println!("{} {} {}", link1, link2, dist);

		let mut node1 = nodes.get(&link1).unwrap_or(&Node { id: link1.clone(), links: Vec::new() });
		let mut node2 = nodes.get(&link2).unwrap_or(&Node { id: link2.clone(), links: Vec::new() });

		// node1.links.push(node2);
		// node2.links.push(node1);

		// nodes.insert(link1, *node1);
		// nodes.insert(link2, *node2);
	}

	println!("Part1: {}", 0);
	println!("Part2: {}", 0);
}
