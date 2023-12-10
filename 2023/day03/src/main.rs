use regex::Regex;
use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let re_numbers = Regex::new(r"\d+").unwrap();
    let re_symbols = Regex::new(r"[#%&*+/=@$%]").unwrap();

    let mut numbers = HashMap::new();
    input.lines().enumerate().for_each(|(i, line)| {
        re_numbers.find_iter(line).for_each(|capture| {
            let mut possible_positions = vec![];
            for ele in capture.start()..capture.end() {
                possible_positions.push((i, ele));
            }
            numbers.insert(
                possible_positions,
                (capture.as_str().parse::<i32>().unwrap(), false),
            );
        });
    });

    let mut part1: u64 = 0;
    input.lines().enumerate().for_each(|(i, line)| {
        re_symbols.find_iter(line).for_each(|capture| {
            let possible_positions = vec![
                (i - 1, capture.start() - 1),
                (i - 1, capture.start()),
                (i - 1, capture.start() + 1),
                (i, capture.start() - 1),
                (i, capture.start() + 1),
                (i + 1, capture.start() - 1),
                (i + 1, capture.start()),
                (i + 1, capture.start() + 1),
            ];
            for (i, j) in possible_positions {
                numbers
                    .iter_mut()
                    .filter(|(key, _)| {
                        return key.contains(&(i, j));
                    })
                    .for_each(|(_, val)| {
                        println!("{:?}", val);
                        if !val.1 {
                            val.1 = true;
                            part1 += val.0 as u64;
                        }
                    });
            }
        });
    });

    println!("Part 1: {:?}", part1);
    // println!("Part 2: {:?}", part2);
}
