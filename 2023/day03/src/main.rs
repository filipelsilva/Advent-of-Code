use regex::Regex;
use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let re_numbers = Regex::new(r"\d+").unwrap();
    let re_symbols = Regex::new(r"[#%&*+/=@$%]").unwrap();

    let mut numbers = HashMap::new();
    input.lines().enumerate().for_each(|(i, line)| {
        re_numbers.find_iter(line).for_each(|capture| {
            for j in capture.start()..capture.end() {
                numbers.insert((i, j), capture.as_str().parse::<i32>().unwrap());
            }
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
            let mut found_numbers = vec![];
            for (i, j) in possible_positions {
                if let Some(number) = numbers.get(&(i, j)) {
                    if !found_numbers.contains(number) {
                        println!("{} {} {}", i, j, number);
                        part1 += *number as u64;
                        found_numbers.push(*number);
                    }
                }
            }
        });
    });

    println!("Part 1: {:?}", part1);
    // println!("Part 2: {:?}", part2);
}
