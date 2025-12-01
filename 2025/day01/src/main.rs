use std::fs;

fn main() {
    let input = fs::read_to_string("input2").unwrap();

    let mut pointer = 50;

    let mut zero_counter = 0;
    let mut zero_counter_pt2 = 0;

    input.lines().for_each(|line| {
        let (direction, distance) = line.split_at(1);
        let number_distance = distance.parse::<i32>().unwrap();

        zero_counter_pt2 += number_distance / 100;

        let new_pointer = (pointer
            + match direction {
                "R" => number_distance,
                "L" => -number_distance,
                _ => panic!("should catch R or L"),
            })
            % 100;

        if pointer < 0 && new_pointer >= 0 {
            zero_counter_pt2 += 1;
        } else if pointer > 0 && new_pointer <= 0 {
            zero_counter_pt2 += 1;
        } else if new_pointer == 0 {
            zero_counter_pt2 += 1;
        }

        if new_pointer == 0 {
            zero_counter += 1;
        }

        println!("{pointer} -> {new_pointer} | {line} | {zero_counter} | {zero_counter_pt2}");

        pointer = new_pointer.rem_euclid(100);
    });

    println!("Part1: {zero_counter}");
    println!("Part2: {zero_counter_pt2}");
}
