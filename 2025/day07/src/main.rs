use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut lines = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut counter_pt1 = 0;
    let mut counter_pt2 = 0;

    for y in 1..lines.len() {
        for x in 0..lines[0].len() {
            match lines[y - 1][x] {
                'S' | '|' => {
                    if lines[y][x] == '.' {
                        lines[y][x] = '|';
                    }
                }
                _ => (),
            }
            match lines[y][x] {
                '^' => {
                    if x > 0 && lines[y][x - 1] != '^' {
                        lines[y][x - 1] = '|';
                    }
                    if x < lines[y].len() - 1 && lines[y][x + 1] != '^' {
                        lines[y][x + 1] = '|';
                    }
                }
                _ => (),
            }
        }
        println!("{}", lines[y].iter().collect::<String>());
    }

    for y in 0..lines.len() {
        for x in 0..lines[0].len() {
            if lines[y][x] == '^' {
                if lines[y - 1][x] == '|' {
                    counter_pt1 += 1;
                }
            }
        }
    }

    println!("Part1: {counter_pt1}");
    println!("Part2: {counter_pt2}");
}
