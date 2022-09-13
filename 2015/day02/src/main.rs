use std::fs;

fn main() {
	let input = fs::read_to_string("input").unwrap();
	let mut part1: usize = 0;
	let mut part2: usize = 0;
	for line in input.lines() {
		if let [l, w, h] = &line
							.split("x")
							.map(|num| num.parse::<usize>().unwrap())
							.collect::<Vec<_>>()[..] {
			let areas = vec![l*w, w*h, h*l];
			let sum_area: usize = areas.iter().sum();
			let min_area = areas.iter().min().unwrap();
			part1 += 2 * sum_area;
			part1 += min_area;

			let volume = l*w*h;
			let perimeters = vec![2*l+2*w, 2*w+2*h, 2*h+2*l];
			let min_perimeter = perimeters.iter().min().unwrap();
			part2 += volume;
			part2 += min_perimeter;
		}
	}
	println!("Part1: {}", part1);
	println!("Part2: {}", part2);
}
