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

        let start_num = start.parse::<u64>().unwrap();
        let end_num = end.parse::<u64>().unwrap();

        for num in start_num..=end_num {
            let istr = format!("{num}");

            let (part1, part2) = istr.split_at(istr.len() / 2);
            if part1 == part2 {
                println!("PT1 FOUND: {num}");
                counter_pt1 += num;
            }

            // let divisors = divisors::get_divisors(istr.len()).push(1);
            // println!("DIVISORS: {divisors:#?}");
            for divisor in 1..=istr.len() / 2 {
                let subs = istr
                    .as_bytes()
                    .chunks(divisor.try_into().unwrap())
                    .map(str::from_utf8)
                    .collect::<Result<Vec<&str>, _>>()
                    .unwrap();

                if subs.windows(2).all(|window| window[0] == window[1]) {
                    println!("PT2 FOUND: {num} {divisor} {subs:#?}");
                    counter_pt2 += num;
                    break;
                }
            }
        }
    }

    println!("Part1: {counter_pt1}");
    println!("Part2: {counter_pt2}");
}
