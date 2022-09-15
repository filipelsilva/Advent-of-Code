use std::fs;

fn main() {
	let input = fs::read_to_string("input").unwrap();

	let mut part1: i32 = 0;
	let mut part2: i32 = 0;

	for line in input.lines() {

		println!("{}", line);

		let code = line.len();
		let mut chars = 0;

		let string: Vec<char> = line[1..line.len()-1].chars().collect::<Vec<char>>();
		let len = string.len();
		println!("{}", string.iter().collect::<String>());

		let mut i = 0;
		loop {
			if i == len {
				break;
			}

			println!("Char: {} {}", i, string[i]);

			match string[i] {
				'\\' => {
					if i != len - 1 {
						match string[i+1] {
							'\\' => chars += 1,
							'"' => (),
							'x' => chars -= 2,
							_ => (),
						}
					}
				},
				_ => chars += 1,
			};

			i += 1;

			println!("{} {}", code, chars);

		}

		// println!("FINAL: {} {}", code, chars);

		part1 += code as i32;
		part1 -= chars as i32;
	}

	println!("Part1: {}", part1);
	println!("Part2: {}", part2);
}
