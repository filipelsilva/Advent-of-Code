use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut game_results = HashMap::new();

    input.lines().for_each(|line| {
        let game_split: Vec<&str> = line.split(": ").collect();
        let game_id = game_split
            .first()
            .unwrap()
            .replace("Game ", "")
            .parse::<usize>()
            .unwrap();

        let mut max_nums = HashMap::new();
        game_split.last().unwrap().split("; ").for_each(|game| {
            game.split(", ").for_each(|color_result| {
                let color_split: Vec<&str> = color_result.split(" ").collect();
                let number = color_split.first().unwrap().parse::<usize>().unwrap();
                let color = color_split.last().unwrap().to_string();

                if max_nums.get(&color).is_none() || max_nums.get(&color).unwrap() < &number {
                    max_nums.insert(color, number);
                }
            })
        });

        game_results.insert(game_id, max_nums);
    });

    let part1: usize = game_results
        .iter()
        .filter(|(_, results)| {
            return results.get("red").unwrap_or(&0) <= &max_red
                && results.get("green").unwrap_or(&0) <= &max_green
                && results.get("blue").unwrap_or(&0) <= &max_blue;
        })
        .map(|(i, _)| i)
        .sum();

    println!("Part 1: {:?}", part1);

    let part2: usize = game_results
        .iter()
        .map(|(_, results)| {
            return results.get("red").unwrap_or(&1)
                * results.get("green").unwrap_or(&1)
                * results.get("blue").unwrap_or(&1);
        })
        .sum();

    println!("Part 2: {:?}", part2);
}
