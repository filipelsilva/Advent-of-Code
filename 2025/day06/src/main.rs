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
        let operands = lines
            .iter()
            .enumerate()
            .map(|(index, line)| {
                if index != op_index {
                    return line[i].parse::<u64>().unwrap();
                }
                return 0;
            })
            .filter(|el| *el != 0)
            .collect::<Vec<_>>();

        let result: u64 = match op {
            "+" => operands.iter().copied().reduce(|acc, el| acc + el).unwrap(),
            "*" => operands.iter().copied().reduce(|acc, el| acc * el).unwrap(),
            _ => 0,
        };

        counter_pt1 += result;
    }

    println!("Part1: {counter_pt1}");
    println!("Part2: {counter_pt2}");
}
