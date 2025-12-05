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

    let mut final_ranges: Vec<RangeInclusive<u64>> = Vec::new();

    ranges.sort_by_key(|r| *r.start());
    for range in ranges {
        if let Some(last) = final_ranges.last_mut() {
            if *range.start() <= *last.end() + 1 {
                let new_end = std::cmp::max(*last.end(), *range.end());
                let new_start = *last.start();
                *last = new_start..=new_end;
            } else {
                final_ranges.push(range);
            }
        } else {
            final_ranges.push(range);
        }
    }

    for range in &final_ranges {
        counter_pt2 += range.clone().count();
    }

    println!("Part1: {counter_pt1}");
    println!("Part2: {counter_pt2}");
}
