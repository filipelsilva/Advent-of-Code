use std::{collections::HashSet, fs};

fn distance(x1: i64, y1: i64, x2: i64, y2: i64) -> u64 {
    let x_delta = (x1 - x2).abs() + 1;
    let y_delta = (y1 - y2).abs() + 1;
    (x_delta * y_delta).try_into().unwrap()
}

fn main() {
    let path = "input";
    let input = fs::read_to_string(path).unwrap();

    let positions = input
        .lines()
        .map(|line| {
            line.trim()
                .split(',')
                .map(|el| el.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut counter_pt1: u64 = 0;
    let mut counter_pt2: u64 = 0;

    let mut board: Vec<Vec<char>> = Vec::new();

    let mut max_x = 0;
    let mut max_y = 0;
    for i in 1..positions.len() - 1 {
        let x1 = positions[i - 1][0];
        let y1 = positions[i - 1][1];
        let x2 = positions[i][0];
        let y2 = positions[i][1];
        if x1 == x2 {
            for y in y1..=y2 {
                valid_positions.insert((y, x1));
            }
        } else if y1 == y2 {
            for x in x1..=x2 {
                valid_positions.insert((y1, x));
            }
        }

        if x1 < min_x {
            min_x = x1;
        }
        if x2 < min_x {
            min_x = x2;
        }
        if y1 < min_y {
            min_y = y1;
        }
        if y2 < min_y {
            min_y = y2;
        }

        if x1 > max_x {
            max_x = x1;
        }
        if x2 > max_x {
            max_x = x2;
        }
        if y1 > max_y {
            max_y = y1;
        }
        if y2 > max_y {
            max_y = y2;
        }
    }

    for i in 0..positions.len() - 1 {
        for j in i + 1..positions.len() {
            let x1 = positions[i][0];
            let y1 = positions[i][1];
            let x2 = positions[j][0];
            let y2 = positions[j][1];

            let new_area = distance(x1, y1, x2, y2);
            if new_area > counter_pt1 {
                counter_pt1 = new_area;
            }

            for y in y1..=y2 {
                for x in x1..=x2 {
                    valid_positions.insert((y, x));
                }
            }
        }
    }

    println!("Part1: {counter_pt1}");
    // println!("Part2: {counter_pt2}");
}
