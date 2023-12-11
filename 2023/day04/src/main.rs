use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut card_matches: HashMap<u32, u32> = HashMap::new();

    input.lines().enumerate().for_each(|(i, line)| {
        let splits: Vec<&str> = line.split(":").last().unwrap().split("|").collect();
        let winning_numbers: HashSet<u32> = splits
            .first()
            .unwrap()
            .split_whitespace()
            .map(|ele| ele.parse::<u32>().unwrap())
            .collect();
        let our_numbers: HashSet<u32> = splits
            .last()
            .unwrap()
            .split_whitespace()
            .map(|ele| ele.parse::<u32>().unwrap())
            .collect();
        // println!("{:?}", (winning_numbers, our_numbers));

        let matches = winning_numbers.intersection(&our_numbers).count();
        card_matches.insert(i as u32 + 1, matches as u32);
    });

    let part1: u64 = card_matches.iter().fold(0, |sum, elem| {
        if *elem.1 > 0 {
            return sum + 2_u64.pow(*elem.1 as u32 - 1);
        } else {
            return sum;
        }
    });

    let mut part2: u64 = 0;
    let mut queue: VecDeque<u32> = card_matches.keys().copied().collect();

    while !queue.is_empty() {
        part2 += 1;
        let card_number = queue.pop_front().unwrap();
        let matches = card_matches.get(&card_number).unwrap();
        if matches != &0 {
            for i in 1..matches + 1 {
                queue.push_back(card_number + i);
            }
        }
        // println!("{:?}", (card_number, matches, queue.clone()));
    }

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}
