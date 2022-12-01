use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut arr = Vec::<u64>::new();
    let mut sum: u64 = 0;
    for character in input.split("\n") {
        if character == "" {
            arr.push(sum);
            sum = 0;
        } else {
            sum += character.parse::<u64>().unwrap();
        }
    }
    arr.sort_unstable();
    println!("Part1: {}", arr[arr.len() - 1]);
    println!("Part2: {}", arr[arr.len() - 3..].iter().sum::<u64>());
}
