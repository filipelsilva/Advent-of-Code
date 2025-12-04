use std::fs;

fn main() {
    let binding = fs::read_to_string("input").unwrap();
    let mut input = binding
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut counter_pt1 = 0;
    let mut counter_pt2 = 0;

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == '@' {
                let mut counter = 0;

                let start_y = match y {
                    0 => 0,
                    _ => y - 1,
                };
                let start_x = match x {
                    0 => 0,
                    _ => x - 1,
                };

                let end_y = if y == input.len() - 1 { y } else { y + 1 };
                let end_x = if x == input[y].len() - 1 { x } else { x + 1 };

                for i in start_y..=end_y {
                    for j in start_x..=end_x {
                        if i >= input.len() && j >= input[y].len() {
                            continue;
                        }
                        if i == y && j == x {
                            continue;
                        }
                        if input[i][j] == '@' {
                            counter += 1;
                        }
                    }
                }

                // println!("{y},{x}: {start_y}-{end_y},{start_x}-{end_x}: {counter}");

                if counter < 4 {
                    counter_pt1 += 1;
                    println!("FOUND {y},{x}")
                }
            }
        }
    }

    loop {
        let last_counter = counter_pt2;

        for y in 0..input.len() {
            for x in 0..input[0].len() {
                if input[y][x] == '@' {
                    let mut counter = 0;

                    let start_y = match y {
                        0 => 0,
                        _ => y - 1,
                    };
                    let start_x = match x {
                        0 => 0,
                        _ => x - 1,
                    };

                    let end_y = if y == input.len() - 1 { y } else { y + 1 };
                    let end_x = if x == input[y].len() - 1 { x } else { x + 1 };

                    for i in start_y..=end_y {
                        for j in start_x..=end_x {
                            if i >= input.len() && j >= input[y].len() {
                                continue;
                            }
                            if i == y && j == x {
                                continue;
                            }
                            if input[i][j] == '@' {
                                counter += 1;
                            }
                        }
                    }

                    if counter < 4 {
                        counter_pt2 += 1;
                        input[y][x] = 'x';
                    }
                }
            }
        }

        if counter_pt2 - last_counter == 0 {
            break;
        }
    }

    println!("Part1: {counter_pt1}");
    println!("Part2: {counter_pt2}");
}
