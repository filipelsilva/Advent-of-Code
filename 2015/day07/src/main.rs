use std::{fs, collections::HashMap};
use std::fmt::Display;

enum Operations {
	Value(String),
	Not(String),
	And(String, String),
	Or(String, String),
	LShift(String, usize),
	RShift(String, usize),
}

impl Display for Operations {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Operations::Value(num) => write!(f, "Value: {}", num),
			Operations::Not(num) => write!(f, "Not: {}", num),
			Operations::And(num, num2) => write!(f, "And: {},{}", num, num2),
			Operations::Or(num, num2) => write!(f, "Or: {},{}", num, num2),
			Operations::LShift(num, offset) => write!(f, "LShift: {},{}", num, offset),
			Operations::RShift(num, offset) => write!(f, "RShift: {},{}", num, offset),
		}
	}
}


fn solve(wires: &HashMap<String, Operations>, values: &mut HashMap<String, u16>, wire: String) -> u16 {
	if values.contains_key(&wire) {
		return *values.get(&wire).unwrap();
	} else if let Ok(number) = wire.parse::<u16>() {
		return number;
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

		// println!("{}: {}", wire, value);

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

		if op1 == "NOT" {
			wires.insert(wire.to_string(), Operations::Not(operator.to_string()));
			continue 'outer;
		}

		let op2 = tokens.next().unwrap();

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

	// for (k, v) in values.iter() {
	// 	println!("Value of {}: {}", k, v);
	// }

	// for (k, v) in wires.iter() {
	// 	println!("Operation of {}: {}", k, v);
	// }

	println!("Part1: {}", solve(&wires, &mut values, "a".to_string()));
}
