use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let split = input.trim().split("\n\n").collect::<Vec<_>>();

    let ranges = split[0]
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

    println!("Part1: {counter_pt1}");
    println!("Part2: {counter_pt2}");
}
