use std::collections::HashMap;
use std::fs;
use std::ops::{Add,AddAssign};
use std::fmt::Display;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coords {
	x: i64,
	y: i64,
}

impl Add for Coords {
	type Output = Coords;
	fn add(self, other: Self) -> Self {
		Self {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}
}

impl AddAssign for Coords {
	fn add_assign(&mut self, other: Self) {
		*self = Self {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}
}

impl Display for Coords {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{},{}", self.x, self.y)
	}
}

fn main() {
	let input = fs::read_to_string("input").unwrap();
	let mut coords1 = Coords { x: 0, y: 0 };
	let mut coords2a = Coords { x: 0, y: 0 };
	let mut coords2b = Coords { x: 0, y: 0 };
	let mut map1: HashMap<Coords, bool> = HashMap::new();
	let mut map2: HashMap<Coords, bool> = HashMap::new();
	let mut is_santa = true;
	let mut part1: usize = 0;
	let mut part2: usize = 0;
	for direction in input.chars() {
		let dir = match direction {
			'^' => Coords { x:  0,  y:  1 },
			'v' => Coords { x:  0,  y: -1 },
			'>' => Coords { x:  1,  y:  0 },
			'<' => Coords { x: -1,  y:  0 },
			_ => Coords { x: 0, y: 0 },
		};
		coords1 += dir;
		if is_santa {
			coords2a += dir;
		} else {
			coords2b += dir;
		}
		if !map1.contains_key(&coords1) {
			map1.insert(coords1, true);
			part1 += 1;
		}
		if is_santa && !map2.contains_key(&coords2a) {
			map2.insert(coords2a, true);
			part2 += 1;
		}
		if !is_santa && !map2.contains_key(&coords2b) {
			map2.insert(coords2b, true);
			part2 += 1;
		}
		is_santa = !is_santa;
	}
	println!("Part1: {}", part1);
	println!("Part2: {}", part2);
}
