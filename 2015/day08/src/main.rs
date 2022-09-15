use std::fs;
use regex::Regex;

fn main() {
	let input = fs::read_to_string("input2").unwrap();

	let mut part1: i32 = 0;
	let mut part2: i32 = 0;

	let patterns = vec![
		Regex::new(r"[^\\]\\\\").unwrap(),
		Regex::new(r#"\\""#).unwrap(),
		Regex::new(r"[^\\]\\x[[:xdigit:]]{2}").unwrap(),
		Regex::new(r"[^\\]\\x[[:xdigit:]]{2}").unwrap(),
		Regex::new(r"[^\\]\\x[[:xdigit:]]{2}").unwrap(),
	];

	for line in input.lines() {
		let code = line.len();

		// FIRST PART
		let mut chars = code - 2; // quotation marks at beginning and end
		for pat in &patterns {
			chars -= pat.find_iter(line).count();
		}

		// println!("{} {},{}", line, code, chars);

		part1 += code as i32;
		part1 -= chars as i32;

		// SECOND PART
		todo!()
	}

	println!("Part1: {}", part1);
	println!("Part2: {}", part2);
}
