use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let lines = input
        .lines()
        .map(|line| line.trim().split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let op_index = lines.len() - 1;

    let mut counter_pt1 = 0;
    let mut counter_pt2 = 0;

    for i in 0..lines[0].len() {
        let op = lines[op_index][i];
        let operands = lines[..op_index]
            .iter()
            .map(|line| line[i].parse::<u64>().unwrap())
            .filter(|el| *el != 0)
            .collect::<Vec<_>>();

        let result: u64 = match op {
            "+" => operands.iter().copied().reduce(|acc, el| acc + el).unwrap(),
            "*" => operands.iter().copied().reduce(|acc, el| acc * el).unwrap(),
            _ => 0,
        };

        counter_pt1 += result;
    }

    let lines_pt2 = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut numbers_vec = Vec::new();
    for i in (0..lines_pt2[0].len()).rev() {
        let op = lines_pt2[op_index][i];
        let operand = lines_pt2[..op_index]
            .iter()
            .map(|line| line[i])
            .filter(|chr| *chr != ' ')
            .collect::<String>();

        if !operand.is_empty() {
            numbers_vec.push(operand.parse::<u64>().unwrap());
        };

        match op {
            '+' => {
                counter_pt2 += numbers_vec
                    .iter()
                    .copied()
                    .reduce(|acc, el| acc + el)
                    .unwrap();
                numbers_vec.clear();
            }
            '*' => {
                counter_pt2 += numbers_vec
                    .iter()
                    .copied()
                    .reduce(|acc, el| acc * el)
                    .unwrap();
                numbers_vec.clear();
            }
            _ => (),
        }
    }

    println!("Part1: {counter_pt1}");
    println!("Part2: {counter_pt2}");
}
