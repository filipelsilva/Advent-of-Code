use std::{fs, collections::HashMap};

fn main() {
	let input = fs::read_to_string("input").unwrap();
	let input = "123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i\n";
	let inits = input.lines().filter(|line| line.split(' ').count() == 3);
	let ops = input.lines().filter(|line| line.split(' ').count() > 3);

	let mut wires: HashMap<&str, u16> = HashMap::new();
	let mut part1: u16 = 0;
	let mut part2: u16 = 0;

	for line in inits {
		println!("{}", line);
		let tokens = &mut line.split(' ');
		let number = tokens.next().unwrap().parse::<u16>().unwrap();
		tokens.next();
		wires.insert(tokens.next().unwrap(), number);
	}

	for line in ops {
		println!("{}", line);
		let tokens = &mut line.split(' ');
		let op1 = tokens.next().unwrap();
		let operation = tokens.next().unwrap();
		let op2 = tokens.next().unwrap();
		let not_wire = tokens.next().unwrap();
		let mut number = match operation {
			"AND"    => wires.get(op1).unwrap() & wires.get(op2).unwrap(),
			"OR"     => wires.get(op1).unwrap() | wires.get(op2).unwrap(),
			"LSHIFT" => wires.get(op1).unwrap() << op2.parse::<u16>().unwrap(),
			"RSHIFT" => wires.get(op1).unwrap() >> op2.parse::<u16>().unwrap(),
			_ => 0,
		};
		if op1 == "NOT" {
			number = !wires.get(operation).unwrap();
			wires.insert(not_wire, number);
		} else {
			wires.insert(tokens.next().unwrap(), number);
		}
	}

	for (k, v) in wires.iter() {
		println!("{} {}", k, v);
	}

	// println!("Part1: {}", wires.get("a").unwrap());
	println!("Part2: {}", part2);
}
