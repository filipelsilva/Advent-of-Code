use std::{fs, collections::HashMap};

enum Operations {
	Value(String),
	Not(String),
	And(String, String),
	Or(String, String),
	LShift(String, usize),
	RShift(String, usize),
}

fn solve(wires: &HashMap<String, Operations>, values: &mut HashMap<String, u16>, wire: String) -> u16 {
	if values.contains_key(&wire) {
		return *values.get(&wire).unwrap();
	} else {
		let op = wires.get(&wire).unwrap();
		let value = match op {
			Operations::Value(num) => match num.parse::<u16>() {
				Ok(number) => number,
				Err(_) => solve(wires, values, num.to_string()),
			},
			Operations::Not(num) => !solve(wires, values, num.to_string()),
			Operations::And(num, num2) => {
				let n = solve(wires, values, num.to_string());
				let n1 = solve(wires, values, num2.to_string());
				n & n1
			},
			Operations::Or(num, num2) => {
				let n = solve(wires, values, num.to_string());
				let n1 = solve(wires, values, num2.to_string());
				n | n1
			},
			Operations::LShift(num, offset) => solve(wires, values, num.to_string()) << offset,
			Operations::RShift(num, offset) => solve(wires, values, num.to_string()) >> offset,
		};
		values.insert(wire, value);
		value
	}
}

fn main() {
	let input = fs::read_to_string("input").unwrap();
	// let input = "123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i\n";

	let mut wires: HashMap<String, Operations> = HashMap::new();
	let mut values: HashMap<String, u16> = HashMap::new();

	'outer: for line in input.lines() {

		// println!("{}", line);

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

		let op1 = tokens.next().unwrap();

		let operator = match tokens.next() {
			Some(tmp) => tmp,
			None => "",
		};

		if operator == "" {
			wires.insert(wire.to_string(), Operations::Value(op1.to_string()));
			continue 'outer;
		}

		let op2 = if op1 == "NOT" {
			""
		} else {
			tokens.next().unwrap()
		};

		// println!("{},{},{}", op1, operator, op2);

		let operation = match operator {
			"NOT" => Operations::Not(op1.to_string()),
			"AND" => Operations::And(op1.to_string(), op2.to_string()),
			"OR" => Operations::Or(op1.to_string(), op2.to_string()),
			"LSHIFT" => Operations::LShift(op1.to_string(), op2.to_string().parse().unwrap()),
			"RSHIFT" => Operations::RShift(op1.to_string(), op2.to_string().parse().unwrap()),
			_ => Operations::Value("0".to_string()),
		};

		wires.insert(wire.to_string(), operation);
	}

	println!("{}", solve(&wires, &mut values, "a".to_string()));
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
