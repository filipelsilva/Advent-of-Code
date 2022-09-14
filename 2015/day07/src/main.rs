use std::{fs, collections::HashMap};

enum Operations {
	Value(String),
	Not(String),
	And(String, String),
	Or(String, String),
	LShift(String, usize),
	RShift(String, usize),
}

fn main() {
	let input = fs::read_to_string("input").unwrap();
	// let input = "123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i\n";

	let mut wires: HashMap<String, Operations> = HashMap::new();
	let mut values: HashMap<String, u16> = HashMap::new();

	'outer: for line in input.lines() {
		println!("{}", line);
		let mut tokens = line.split(" -> ");
		let calculate = tokens.next().unwrap();
		let wire = tokens.next().unwrap();

		// If it is a simple assignment
		match calculate.parse::<u16>() {
			Ok(number) => {
				values.insert(wire.to_string(), number);
				continue 'outer;
			},
			Err(_) => (),
		}

		// If it isn't
		let tokens = &mut calculate.split(" ");

		let mut op1 = "";
		let mut op2 = "";
		let mut operator = tokens.next().unwrap();

		if tokens.count() == 1 {
			wires.insert(wire.to_string(), Operations::Value(operator.to_string()));
			continue 'outer;
		}

		if operator == "NOT" {
			op1 = tokens.next().unwrap();
		} else {
			op1 = operator;
			operator = tokens.next().unwrap();
			op2 = tokens.next().unwrap();
		}

		let operation = match operator {
			"AND" => Operations::And(op1.to_string(), op2.to_string()),
			"OR" => Operations::Or(op1.to_string(), op2.to_string()),
			"LSHIFT" => Operations::LShift(op1.to_string(), op2.to_string().parse().unwrap()),
			"RSHIFT" => Operations::And(op1.to_string(), op2.to_string().parse().unwrap()),
			_ => Operations::Value("0".to_string()),
		};

		wires.insert(wire.to_string(), operation);
	}

	// for (k, v) in wires.iter() {
	// 	println!("{} {}", k, v);
	// }

	// for line in ops {
	// 	println!("{}", line);
	// 	let tokens = &mut line.split(' ');
	// 	let op1 = tokens.next().unwrap();
	// 	let operation = tokens.next().unwrap();
	// 	let op2 = tokens.next().unwrap();
	// 	let not_wire = tokens.next().unwrap();
	// 	let mut number = match operation {
	// 		"AND"    => wires.get(op1).unwrap() & wires.get(op2).unwrap(),
	// 		"OR"     => wires.get(op1).unwrap() | wires.get(op2).unwrap(),
	// 		"LSHIFT" => wires.get(op1).unwrap() << op2.parse::<u16>().unwrap(),
	// 		"RSHIFT" => wires.get(op1).unwrap() >> op2.parse::<u16>().unwrap(),
	// 		_ => 0,
	// 	};
	// 	if op1 == "NOT" {
	// 		number = !wires.get(operation).unwrap();
	// 		wires.insert(not_wire, number);
	// 	} else {
	// 		wires.insert(tokens.next().unwrap(), number);
	// 	}
	// }

	// println!("Part1: {}", wires.get("a").unwrap());
	// println!("Part2: {}", part2);
}

fn test1() {
	let input = fs::read_to_string("input").unwrap();
	// let input = "123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i\n";

	let mut wires: HashMap<&str, u16> = HashMap::new();

	let mut inits = input.lines().filter(|line| line.split(' ').count() == 3).collect::<Vec<&str>>();
	inits.sort();

	let mut ops = input.lines().filter(|line| line.split(' ').count() > 3).collect::<Vec<&str>>();
	ops.sort();

	for line in inits {
		// println!("{}", line);
		let tokens = &mut line.split(' ');
		let first = tokens.next().unwrap();
		tokens.next();
		let second = tokens.next().unwrap();

		match first.parse::<u16>() {
			Ok(number) => {
				wires.insert(second, number);
			},
			Err(_) => {
				match wires.get(first) {
					Some(number) => {
						wires.insert(second, *number);
					},
					None => {
						wires.insert(first, 0);
						wires.insert(second, 0);
					},
				};
			},
		};
	}

	for (k, v) in wires.iter() {
		println!("{} {}", k, v);
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

	// println!("Part1: {}", wires.get("a").unwrap());
	// println!("Part2: {}", part2);
}
