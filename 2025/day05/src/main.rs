use std::{fs, ops::RangeInclusive};

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
            return range[0]..=range[1];
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
            if range.contains(&ingredient) {
                counter_pt1 += 1;
                break;
            }
        }
    }

    let mut final_ranges: Vec<RangeInclusive<u64>> = vec![];
    loop {
        for range_to_add in &ranges {
            if final_ranges.is_empty() {
                final_ranges.push(range_to_add.clone());
                continue;
            }

            let mut should_add = true;

            for i in 0..final_ranges.len() {
                if final_ranges[i].contains(range_to_add.start()) {
                    if !final_ranges[i].contains(range_to_add.end()) {
                        final_ranges[i] = *range_to_add.start()..=*range_to_add.end()
                    }
                    should_add = false;
                } else if final_ranges[i].contains(range_to_add.end()) {
                    if !final_ranges[i].contains(range_to_add.start()) {
                        final_ranges[i] = *range_to_add.start()..=*range_to_add.end()
                    }
                    should_add = false;
                }

                if !should_add {
                    break;
                }
            }

            if should_add {
                final_ranges.push(range_to_add.clone());
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
        counter_pt2 += range.count();
    }

    println!("Part1: {counter_pt1}");
    println!("Part2: {counter_pt2}");
}
