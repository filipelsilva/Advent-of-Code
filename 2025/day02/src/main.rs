use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let ranges = input.trim().split(',').collect::<Vec<_>>();

    let mut counter_pt1 = 0;
    let mut counter_pt2 = 0;

    for range in ranges {
        let split = range.split('-').collect::<Vec<_>>();

        let start = split[0];
        let end = split[1];

        println!("RANGE: {start} - {end}");

        if start.len() % 2 != 0 && end.len() % 2 != 0 {
            println!("SKIP (ODD LEN RANGE)");
            continue;
        }

        let start_num = match start.len() % 2 {
            0 => start.parse::<u64>().unwrap(),
            _ => {
                let new_start = format!("1{}", "0".repeat(start.len()));
                new_start.parse::<u64>().unwrap()
            }
        };

        let end_num = match end.len() % 2 {
            0 => end.parse::<u64>().unwrap(),
            _ => {
                let new_end = "9".repeat(end.len() - 1);
                new_end.parse::<u64>().unwrap()
            }
        };

        // let start_num = start.parse::<u64>().unwrap();
        // let end_num = end.parse::<u64>().unwrap();

        println!("RANGE (OPTIMIZED): {start_num} - {end_num}");

        for i in start_num..=end_num {
            let istr = format!("{i}");

            let (part1, part2) = istr.split_at(istr.len() / 2);
            if part1 == part2 {
                println!("FOUND: {i}");
                counter_pt1 += i;
            }
        }
    }

    println!("Part1: {counter_pt1}");
    println!("Part2: {counter_pt2}");
}
