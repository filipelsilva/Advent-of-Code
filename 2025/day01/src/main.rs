use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut pointer = 50;

    let mut zero_counter = 0;
    let mut zero_counter_pt2 = 0;

    input.lines().for_each(|line| {
        let (direction, distance) = line.split_at(1);
        let mut number_distance = distance.parse::<i32>().unwrap();

        zero_counter_pt2 += number_distance / 100;
        number_distance %= 100;

        let new_pointer = (pointer
            + match direction {
                "R" => number_distance,
                "L" => -number_distance,
                _ => panic!("should catch R or L"),
            })
        .rem_euclid(100);

        if new_pointer == 0 {
            zero_counter += 1;
            zero_counter_pt2 += 1;
        } else if pointer != 0 {
            let dist = (new_pointer - pointer).abs();

            if dist != number_distance {
                zero_counter_pt2 += 1;
            }

            if dist == 50 && pointer != 0 {
                if new_pointer < pointer && direction == "R" {
                    zero_counter_pt2 += 1;
                } else if new_pointer > pointer && direction == "L" {
                    zero_counter_pt2 += 1;
                }
            }
        }

        println!("{pointer} -> {new_pointer} | {line} | {zero_counter} | {zero_counter_pt2}");

        pointer = new_pointer;
    });

    println!("Part1: {zero_counter}");
    println!("Part2: {zero_counter_pt2}");
}
