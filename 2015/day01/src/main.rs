use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let indices = input.char_indices();
    let mut part1: isize = 0;
    let mut part2: usize = 0;
    for character in indices {
        match character.1 {
            '(' => part1 += 1,
            ')' => part1 -= 1,
            _ => (),
        };
        if part1 == -1 && part2 == 0 {
            part2 = character.0 + 1;
        }
    }
    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}
