use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let array_numbers = input
        .lines()
        .map(|s| s.chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let numbers = array_numbers
        .iter()
        .map(|s| {
            let first = s.first().unwrap().to_string().parse::<u64>().unwrap();
            let last = s.last().unwrap().to_string().parse::<u64>().unwrap();
            return first * 10 + last;
        })
        .collect::<Vec<_>>();

    println!("Part 1: {:?}", numbers.iter().sum::<u64>());

    let array_numbers2 = input
        .lines()
        .map(|s| {
            let mut array: Vec<(usize, usize)> = vec![];
            for (num, str) in [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ]
            .iter()
            .enumerate()
            {
                s.match_indices(str)
                    .map(|(index, _)| (index, num + 1))
                    .for_each(|(index, x)| array.push((x, index)));
            }
            for num in 1..10 {
                s.match_indices(&num.to_string())
                    .map(|(index, _)| (index, num))
                    .for_each(|(index, x)| array.push((x, index)));
            }
            return array;
        })
        .collect::<Vec<_>>();

    let numbers2 = array_numbers2
        .iter()
        .map(|s| {
            let max_index = s.iter().map(|(_, index)| index).max().unwrap();
            let min_index = s.iter().map(|(_, index)| index).min().unwrap();
            let (max, _) = s
                .iter()
                .filter(|&(_, index)| index == max_index)
                .next()
                .unwrap();
            let (min, _) = s
                .iter()
                .filter(|&(_, index)| index == min_index)
                .next()
                .unwrap();
            return min * 10 + max;
        })
        .collect::<Vec<_>>();

    println!("Part 2: {:?}", numbers2.iter().sum::<usize>());
}
