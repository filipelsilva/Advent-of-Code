use std::{fs, collections::HashSet};

fn main() {
    // let input = fs::read_to_string("input").unwrap();
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
    let lines = input.trim().split("\n");
    let mut part1 = 0;
    let mut part2 = 0;
    let mut groups: Vec<HashSet<char>> = Vec::with_capacity(3);
    for backpack in lines {
        if groups.len() != 3 {
            groups.push(HashSet::from_iter(backpack.chars()));
        } else {
            let inter1: HashSet<&char> = HashSet::from_iter(groups[0].intersection(&groups[1]));
            let inter2: HashSet<&char> = HashSet::from_iter(groups[1].intersection(&groups[2]));
            let intersection = inter1.intersection(&inter2);
            for el in intersection {
                let mut ord = *(*el) as u32;
                if ord > 90 {
                    ord -= 96;
                } else {
                    ord -= 38;
                };
                part2 += ord;
                println!("{} -> {}", el, ord);
            }
            groups.clear();
        }

        let (com1, com2) = backpack.split_at(backpack.len()/2);
        let set1: HashSet<char> = HashSet::from_iter(com1.chars());
        let set2: HashSet<char> = HashSet::from_iter(com2.chars());
        let intersection = set1.intersection(&set2);
        // println!("{} --- {}", com1, com2);
        for el in intersection {
            let mut ord = *el as u32;
            if ord > 90 {
                ord -= 96;
            } else {
                ord -= 38;
            };
            part1 += ord;
            // println!("{} -> {}", el, ord);
        }
    }
    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}
