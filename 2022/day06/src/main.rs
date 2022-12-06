use std::{fs, collections::HashSet};

fn main() {
    let input: Vec<char> = fs::read_to_string("input").unwrap().chars().collect();
    let mut testing1 = HashSet::new();
    let mut testing2 = HashSet::new();
    for i in 4..input.len() {
        testing1.clear();
        (i-4..i).for_each(|x| {
            testing1.insert(input[x]);
        });
        if testing1.len() == 4 {
            println!("Part1: {}", i);
            break;
        }
    }
    for i in 14..input.len() {
        testing2.clear();
        (i-14..i).for_each(|x| {
            testing2.insert(input[x]);
        });
        if testing2.len() == 14 {
            println!("Part1: {}", i);
            break;
        }
    }
}
