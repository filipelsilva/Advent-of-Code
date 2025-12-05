use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let split = input.trim().split("\n\n").collect::<Vec<_>>();

    let mut ranges = split[0]
        .lines()
        .map(|el| {
            let range = el
                .split("-")
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            return (range[0], range[1]);
        })
        .collect::<Vec<_>>();

    let ingredients = split[1]
        .lines()
        .map(|el| el.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut counter_pt1 = 0;
    let mut counter_pt2 = 0;

    for ingredient in ingredients {
        for range in &ranges {
            if ingredient >= range.0 && ingredient <= range.1 {
                counter_pt1 += 1;
                break;
            }
        }
    }

    let mut final_ranges: Vec<(u64, u64)> = vec![];
    loop {
        for range_to_add in &ranges {
            if final_ranges.is_empty() {
                final_ranges.push(*range_to_add);
                continue;
            }

            let mut should_add = true;
            for final_range in &mut final_ranges {
                if range_to_add.0 == final_range.0 && range_to_add.1 == final_range.1 {
                    should_add = false;
                } else if range_to_add.0 == range_to_add.1
                    && (range_to_add.0 == final_range.0 || range_to_add.0 == final_range.1)
                {
                    should_add = false;
                } else if range_to_add.0 < final_range.0 && range_to_add.1 > final_range.1 {
                    should_add = false;
                    final_range.0 = range_to_add.0;
                    final_range.1 = range_to_add.1;
                } else if range_to_add.0 >= final_range.0
                    && range_to_add.0 <= final_range.1
                    && range_to_add.1 >= final_range.1
                {
                    should_add = false;
                    final_range.1 = range_to_add.1;
                } else if range_to_add.1 <= final_range.1
                    && range_to_add.1 >= final_range.0
                    && range_to_add.0 <= final_range.0
                {
                    should_add = false;
                    final_range.0 = range_to_add.0;
                }

                if should_add == false {
                    break;
                }
            }

            if should_add {
                final_ranges.push(*range_to_add);
            }
        }

        if ranges.len() == final_ranges.len() {
            break;
        }

        ranges = final_ranges.clone();
        final_ranges.clear();
    }

    println!("{final_ranges:#?}");
    for range in final_ranges {
        counter_pt2 += (range.1 - range.0) + 1;
    }

    println!("Part1: {counter_pt1}");
    println!("Part2: {counter_pt2}");
}
