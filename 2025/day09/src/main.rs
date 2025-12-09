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

    let mut board_edges: Vec<HashSet<(i64, i64)>> = Vec::new();

    for i in 1..positions.len() - 1 {
        let x1 = positions[i - 1][0];
        let y1 = positions[i - 1][1];
        let x2 = positions[i][0];
        let y2 = positions[i][1];

        assert!((x1 == x2) ^ (y1 == y2));

        let edge: HashSet<(i64, i64)>;
        if x1 == x2 {
            if y1 > y2 {
                edge = (y2..=y1).map(|y| (y, x1)).collect::<HashSet<_>>();
            } else {
                edge = (y1..=y2).map(|y| (y, x1)).collect::<HashSet<_>>();
            }
        } else {
            if x1 > x2 {
                edge = (x2..=x1).map(|x| (y1, x)).collect::<HashSet<_>>();
            } else {
                edge = (x1..=x2).map(|x| (y1, x)).collect::<HashSet<_>>();
            }
        }

        board_edges.push(edge);
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

            // generate edges
            let mut edges: Vec<HashSet<(i64, i64)>> = Vec::new();

            // (y1,x1) ------ (y1,x2)
            //    |              |
            //    |              |
            //    |              |
            // (y2,x1) ------ (y2,x2)
            let top_edge = (x1.min(x2)..=x1.max(x2))
                .map(|x| (y1, x))
                .collect::<HashSet<_>>();
            let bottom_edge = (x1.min(x2)..=x1.max(x2))
                .map(|x| (y2, x))
                .collect::<HashSet<_>>();
            let left_edge = (y1.min(y2)..=y1.max(y2))
                .map(|y| (y, x1))
                .collect::<HashSet<_>>();
            let right_edge = (y1.min(y2)..=y1.max(y2))
                .map(|y| (y, x2))
                .collect::<HashSet<_>>();
            edges.push(top_edge);
            edges.push(bottom_edge);
            edges.push(left_edge);
            edges.push(right_edge);

            // now, check edges
        }
    }

    println!("Part1: {counter_pt1}");
    // println!("Part2: {counter_pt2}");
}
