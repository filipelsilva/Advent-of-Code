use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut parts = input.trim_end().split("\n\n");
    let starting = parts.next().unwrap();

    // parse the initial arrangement
    let crates_tmp = starting.split("\n");
    let mut crates_transposed: Vec<Vec<String>> = Vec::new();
    for line in crates_tmp {
        crates_transposed.push(line.as_bytes()
            .chunks(4)
            .map(std::str::from_utf8)
            .map(|part| {
                Ok::<String, String>(part.unwrap()
                    .replace(" ", "")
                    .replace("]", "")
                    .replace("[", ""))
            })
            .collect::<Result<Vec<String>, _>>()
            .unwrap()
        );
    }
    let mut crates : Vec<Vec<String>> = (0..crates_transposed[0].len()).map(|i| {
        crates_transposed.iter()
            .map(|c| c[i].as_str().to_string())
            .rev().skip(1)
            .filter(|test| test != "").collect()
    }).collect();
    // println!("{:?}", crates);
    let mut copy = crates.to_vec();

    // parse the moves
    let mut moves = Vec::new();
    let moves_tmp = parts.next().unwrap().split("\n");
    moves_tmp.for_each(|part: &str| {
        let mut single_move = Vec::new();
        part.replace("move ", "").replace("from ", "").replace(" to", "")
            .split(" ").for_each(|part: &str| {
                single_move.push(part.parse::<usize>().unwrap());
            });
        moves.push(single_move);
    });
    // println!("{:?}", moves);

    // part1
    for el in moves.clone() {
        let (count, start, end) = (el[0], el[1], el[2]);
        // println!("{} {} {}", count, start, end);

        // part2
        let removed = copy[start-1][copy[start-1].len()-count..].to_vec();
        copy[end - 1].extend(removed);

        // part1
        (0..count).for_each(|_| {
            let removed = crates[start-1].pop().unwrap();
            crates[end - 1].push(removed);
            copy[start-1].pop(); // this is part2
        });
    }

    println!("Part1: {}", crates.iter().map(|c| c[c.len()-1].as_str()).collect::<String>());
    println!("Part2: {}", copy.iter().map(|c| c[c.len()-1].as_str()).collect::<String>());
}
