use md5;

fn main() {
	let input = "yzbqklnj";
	let mut i: usize = 0;
	let mut part1 = false;
	let mut part2 = false;
	let mut hash: String;
	loop {
		hash = format!("{:x}", md5::compute((input.to_string() + &i.to_string()).as_bytes()));
		if !part1 && &hash[..5] == "00000" {
			println!("Part1: {}", i);
			part1 = !part1;
		}
		if !part2 && &hash[..6] == "000000" {
			println!("Part2: {}", i);
			part2 = !part2;
		}
		if part1 && part2 {
			break;
		}
		i += 1;
	};
}
