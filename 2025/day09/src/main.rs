use std::{collections::HashMap, fs};

fn area(x1: i64, y1: i64, x2: i64, y2: i64) -> u64 {
    let x_delta = (x1 - x2).abs() + 1;
    let y_delta = (y1 - y2).abs() + 1;
    (x_delta * y_delta).try_into().unwrap()
}

fn main() {
    let input = fs::read_to_string("input").unwrap();

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

    for i in 0..positions.len() - 1 {
        for j in i + 1..positions.len() {
            let x1 = positions[i][0];
            let y1 = positions[i][1];
            let x2 = positions[j][0];
            let y2 = positions[j][1];

            let new_area = area(x1, y1, x2, y2);
            if new_area > counter_pt1 {
                counter_pt1 = new_area;
            }
        }
    }

    // https://www.reddit.com/r/adventofcode/comments/1pikbpf/2025_day_9_part_2_my_general_trick_for_this_kind/
    let mut x_positions = positions.iter().map(|el| el[0]).collect::<Vec<_>>();
    let mut y_positions = positions.iter().map(|el| el[1]).collect::<Vec<_>>();
    x_positions.sort();
    y_positions.sort();
    x_positions.dedup();
    y_positions.dedup();

    let x_pos_map = x_positions.iter().enumerate().collect::<HashMap<_, _>>();
    let y_pos_map = y_positions.iter().enumerate().collect::<HashMap<_, _>>();

    // println!("{x_pos_map:#?} {y_pos_map:#?}");

    let new_positions = positions
        .iter()
        .map(|el| {
            let x = el[0];
            let y = el[1];

            let new_x = x_pos_map.iter().find(|el| **el.1 == x).unwrap().0;
            let new_y = y_pos_map.iter().find(|el| **el.1 == y).unwrap().0;

            return (*new_y, *new_x);
        })
        .collect::<Vec<_>>();

    // for i in 0..new_positions.len() {
    //     println!("{:?} {:?}", positions[i], new_positions[i]);
    // }

    let mut board = (0..y_pos_map.len())
        .map(|_| (0..x_pos_map.len()).map(|_| '.').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for i in 1..new_positions.len() {
        let y1 = new_positions[i - 1].0;
        let x1 = new_positions[i - 1].1;
        let y2 = new_positions[i].0;
        let x2 = new_positions[i].1;

        assert!((x1 == x2) ^ (y1 == y2));

        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                board[y][x1] = 'X';
            }
        } else {
            for x in x1.min(x2)..=x1.max(x2) {
                board[y1][x] = 'X';
            }
        }
    }

    for line in &board {
        println!("{}", line.iter().collect::<String>());
    }
    println!("");

    // TODO need to find two islands here, at least
    for y in 0..board.len() {
        let mut intersections = Vec::new();

        for x in 1..board[0].len() {
            if board[y][x - 1] != board[y][x] {
                intersections.push(x);
            }
        }

        // If odd number of intersections → shape is not closed; skip
        if intersections.len() % 2 != 0 {
            println!("WTF");
            continue;
        }

        // Fill pairs of intersections
        for pair in intersections.chunks(2) {
            let start = pair[0];
            let end = pair[1];

            for x in start..end {
                board[y][x] = 'X';
            }
        }
    }

    for line in &board {
        println!("{}", line.iter().collect::<String>());
    }
    println!("");

    for pos in &new_positions {
        board[pos.0][pos.1] = '#';
    }

    // for line in &board {
    //     println!("{}", line.iter().collect::<String>());
    // }
    // println!("");

    for i in 0..new_positions.len() - 1 {
        for j in i + 1..new_positions.len() {
            let y1 = new_positions[i].0;
            let x1 = new_positions[i].1;
            let y2 = new_positions[j].0;
            let x2 = new_positions[j].1;

            let mut should_skip = false;
            for y in y1.min(y2)..=y1.max(y2) {
                for x in x1.min(x2)..=x1.max(x2) {
                    if board[y][x] != 'X' && board[y][x] != '#' {
                        should_skip = true;
                    }
                }
            }

            if should_skip {
                continue;
            }

            let orig_y1 = y_pos_map.get(&y1).unwrap();
            let orig_y2 = y_pos_map.get(&y2).unwrap();
            let orig_x1 = x_pos_map.get(&x1).unwrap();
            let orig_x2 = x_pos_map.get(&x2).unwrap();
            let new_area = area(**orig_x1, **orig_y1, **orig_x2, **orig_y2);

            // println!("({orig_x1}, {orig_y1}) ({orig_x2}, {orig_y2}) - {new_area}");

            if new_area > counter_pt2 {
                counter_pt2 = new_area;
            }
        }
    }

    println!("Part1: {counter_pt1}");
    println!("Part2: {counter_pt2}");
}
