use std::fs;

fn main() {
    let input = fs::read_to_string("input2").unwrap();

    let mut pointer = 50;

    let mut zero_counter = 0;
    let mut zero_counter_pt2 = 0;

    input.lines().for_each(|line| {
        let (direction, distance) = line.split_at(1);
        let number_distance = distance.parse::<i32>().unwrap();

        match direction {
            "R" => pointer += number_distance,
            "L" => pointer -= number_distance,
            _ => panic!("should catch R or L"),
        }

        match pointer {
            ..0 => {
                if pointer.abs() != number_distance {
                    zero_counter_pt2 += (pointer.abs() / 100) + 1;
                }
            }
            101.. => zero_counter_pt2 += pointer / 100,
            _ => (),
        }

        pointer = pointer.rem_euclid(100);

        if pointer == 0 {
            zero_counter += 1;
            zero_counter_pt2 += 1;
        }

        println!("{line} ({pointer}): {zero_counter} | {zero_counter_pt2}");
    });

    println!("Part1: {zero_counter}");
    println!("Part2: {zero_counter_pt2}");
}
