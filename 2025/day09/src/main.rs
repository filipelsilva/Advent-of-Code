use std::fs;

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

    let mut sizes: Vec<((usize, usize), u64)> = Vec::new();
    for i in 0..positions.len() - 1 {
        for j in i + 1..positions.len() {
            let dist = distance(
                positions[i][0],
                positions[i][1],
                positions[j][0],
                positions[j][1],
            );
            sizes.push(((i, j), dist));
        }
    }

    sizes.sort_by(|a, b| b.1.cmp(&a.1));

    let counter_pt1 = sizes[0].1;

    println!("Part1: {counter_pt1}");
    // println!("Part2: {counter_pt2}");
}
