use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let banks = input.trim().lines().collect::<Vec<_>>();

    let mut counter_pt1 = 0;
    let mut counter_pt2 = 0;

    for bank in banks {
        let splits = bank.char_indices().collect::<Vec<_>>();

        let mut max1: (usize, char) = (0, '0');
        for split in &splits {
            if split.0 == splits.len() - 1 {
                continue;
            }

            if split.1 > max1.1 {
                max1 = *split;
            }
        }

        let mut max2: (usize, char) = (0, '0');
        for split in splits {
            if split.0 <= max1.0 {
                continue;
            }

            if split.1 > max2.1 {
                max2 = split;
            }
        }

        let number_str = format!("{}{}", max1.1, max2.1);
        let number = number_str.parse::<usize>().unwrap();

        counter_pt1 += number;

        println!("FOUND: {number} ({max1:#?} - {max2:#?})");
    }

    println!("Part1: {counter_pt1}");
    println!("Part2: {counter_pt2}");
}
