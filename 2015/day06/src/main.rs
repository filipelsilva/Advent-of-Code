use std::fs;

fn part1(input: &String) -> usize {
	let mut part1: usize = 0;
	'outer: for line in input.lines() {
		for pattern in ["ab", "cd", "pq", "xy"] {
			if line.contains(pattern) {
				continue 'outer;
			}
		}
		if line.matches(['a', 'e', 'i', 'o', 'u']).count() < 3 {
			continue;
		}
		let mut double = false;
		for letter in 'a'..='z' {
			let test = letter.to_string().repeat(2);
			if line.contains(&test) {
				double = true;
				break;
			}
		}
		if !double {
			continue;
		}
		part1 += 1;
	}
	return part1;
}

fn part2(input: &String) -> usize {
	let mut part2: usize = 0;
	for line in input.lines() {
		let tmp = line.chars().collect::<Vec<char>>();
		let mut repeat_between = false;
		for i in 0..tmp.len()-2 {
			if tmp[i] == tmp[i+2] {
				repeat_between = true;
				break;
			}
		}
		if !repeat_between {
			continue;
		}
		let mut repeat_pair = false;
		for i in 0..tmp.len()-2 {
			let separator = &tmp[i..i+2].into_iter().collect::<String>();
			if line.split(separator).count() > 2 {
				repeat_pair = true;
			}
		}
		if !repeat_pair {
			continue;
		}
		part2 += 1;
	}
	return part2;
}

fn main() {
	let input = fs::read_to_string("input").unwrap();
	println!("Part1: {}", part1(&input));
	println!("Part2: {}", part2(&input));
}
