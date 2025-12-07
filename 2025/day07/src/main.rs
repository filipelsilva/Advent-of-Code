use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input2").unwrap();

    let mut lines = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut counter_pt1 = 0;

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

    let mut cache: HashMap<(usize, usize), u64> = HashMap::new();

    let counter_pt2 = count_possibilities(
        &lines,
        &mut cache,
        1,
        lines[0].iter().position(|el| *el == 'S').unwrap(),
    );

    println!("Part1: {counter_pt1}");
    println!("Part2: {counter_pt2}");
}

fn count_possibilities(
    lines: &Vec<Vec<char>>,
    cache: &mut HashMap<(usize, usize), u64>,
    y: usize,
    x: usize,
) -> u64 {
    assert!(lines[y][x] == '|');
    println!("({y}, {x})");

    if cache.contains_key(&(y, x)) {
        return *cache.get(&(y, x)).unwrap();
    }

    let mut i = y;
    loop {
        if i == lines.len() - 1 {
            break;
        }

        match lines[i][x] {
            '|' => i += 1,
            '^' => {
                let val = count_possibilities(lines, cache, i + 1, x - 1)
                    + count_possibilities(lines, cache, i + 1, x + 1);
                cache.insert((i, x), val);
                return val;
            }
            _ => (),
        }
    }

    cache.insert((i, x), 1);
    return 1;
}
