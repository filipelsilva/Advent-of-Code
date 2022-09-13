use std::fs;

fn main() {
	let input = fs::read_to_string("input").unwrap();
	let mut part1: u64 = 0;
	let mut part2: u64 = 0;
	let mut grid1 = [[0; 1000]; 1000];
	let mut grid2 = [[0; 1000]; 1000];
	for line in input.lines() {
		let mut matches = line.split(|pattern| pattern == ' ' || pattern == ',');
		let mut action = matches.next().unwrap();
		if action == "turn" {
			action = matches.next().unwrap();
		}
		let x1: usize = matches.next().unwrap().parse().unwrap();
		let y1: usize = matches.next().unwrap().parse().unwrap();
		matches.next(); // ignore the "through"
		let x2: usize = matches.next().unwrap().parse().unwrap();
		let y2: usize = matches.next().unwrap().parse().unwrap();

		for row in &mut grid1[x1..=x2] {
			for light in &mut row[y1..=y2] {
				*light = match action {
					"on" => 1,
					"off" => 0,
					"toggle" => if *light == 1 {0} else {1}, 
					_ => 0
				}
			}
		}

		for row in &mut grid2[x1..=x2] {
			for light in &mut row[y1..=y2] {
				*light += match action {
					"on" => 1,
					"off" => if *light == 0 { 0 } else { -1 },
					"toggle" => 2,
					_ => 0
				}
			}
		}
	}

	for row in &mut grid1[..] {
		for light in &mut row[..] {
			part1 += *light as u64;
		}
	}

	for row in &mut grid2[..] {
		for light in &mut row[..] {
			part2 += *light as u64;
		}
	}

	println!("Part1: {}", part1);
	println!("Part2: {}", part2);
}
