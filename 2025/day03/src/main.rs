use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let banks = input.trim().lines().collect::<Vec<_>>();

    let mut counter_pt1 = 0;
    let mut counter_pt2 = 0;

    for bank in banks {
        let splits = bank.char_indices().collect::<Vec<_>>();

        let mut vec_nums: Vec<(usize, char)> = vec![];

        for i in (0..=1).rev() {
            let mut max: (usize, char) = (0, '0');

            for split in &splits {
                if !vec_nums.is_empty() && split.0 <= vec_nums.last().unwrap().0 {
                    continue;
                }

                if split.0 >= splits.len() - i {
                    continue;
                }

                if split.1 > max.1 {
                    max = *split;
                }
            }

            vec_nums.push(max);
        }

        let number_str = vec_nums.iter().map(|el| el.1).collect::<String>();
        let number = number_str.parse::<usize>().unwrap();

        println!("PT1 FOUND: {number}");
        counter_pt1 += number;

        // Part 2
        vec_nums.clear();

        for i in (0..=11).rev() {
            let mut max: (usize, char) = (0, '0');

            for split in &splits {
                if !vec_nums.is_empty() && split.0 <= vec_nums.last().unwrap().0 {
                    continue;
                }

                if split.0 >= splits.len() - i {
                    continue;
                }

                if split.1 > max.1 {
                    max = *split;
                }
            }

            vec_nums.push(max);
        }

        let number_str = vec_nums.iter().map(|el| el.1).collect::<String>();
        let number = number_str.parse::<usize>().unwrap();

        println!("PT2 FOUND: {number}");
        counter_pt2 += number;
    }

    println!("Part1: {counter_pt1}");
    println!("Part2: {counter_pt2}");
}
