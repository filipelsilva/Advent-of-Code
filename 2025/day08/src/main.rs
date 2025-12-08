use std::fs;

fn distance(x1: i64, y1: i64, z1: i64, x2: i64, y2: i64, z2: i64) -> u64 {
    let x_delta = x1 - x2;
    let y_delta = y1 - y2;
    let z_delta = z1 - z2;
    let squared_delta: u64 = (x_delta.pow(2) + y_delta.pow(2) + z_delta.pow(2))
        .try_into()
        .unwrap();

    squared_delta.isqrt()
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

    let mut distances: Vec<((usize, usize), u64)> = Vec::new();
    for i in 0..positions.len() - 1 {
        for j in i + 1..positions.len() {
            let dist = distance(
                positions[i][0],
                positions[i][1],
                positions[i][2],
                positions[j][0],
                positions[j][1],
                positions[j][2],
            );
            distances.push(((i, j), dist));
        }
    }

    distances.sort_by(|a, b| a.1.cmp(&b.1));

    let mut buckets: Vec<Vec<usize>> = Vec::new();

    let range = match path {
        "input" => 1000,
        _ => 10,
    };

    let mut counter_pt1 = 0;

    let mut i = 0;
    let mut points;
    loop {
        points = distances[i].0;

        let buckets_to_add = buckets
            .iter()
            .filter(|bucket| bucket.contains(&points.0) || bucket.contains(&points.1))
            .map(|bucket| bucket.iter().copied().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        buckets = buckets
            .iter()
            .filter(|bucket| !bucket.contains(&points.0) && !bucket.contains(&points.1))
            .map(|bucket| bucket.iter().copied().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut new_bucket = vec![points.0, points.1];

        if buckets_to_add.len() != 0 {
            for bucket in buckets_to_add {
                for point in bucket {
                    if !new_bucket.contains(&point) {
                        new_bucket.push(point);
                    }
                }
            }
        }

        buckets.push(new_bucket);

        if i == range - 1 {
            buckets.sort_by(|a, b| b.len().cmp(&a.len()));
            counter_pt1 = (buckets[0].len() * buckets[1].len() * buckets[2].len()) as u64;
        }

        if buckets.len() == 1 && buckets[0].len() == positions.len() {
            break;
        }

        i += 1;
    }

    // println!("{buckets:#?}");

    let counter_pt2 = positions[points.0][0] * positions[points.1][0];

    println!("Part1: {counter_pt1}");
    println!("Part2: {counter_pt2}");
}
