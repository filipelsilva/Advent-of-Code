use std::{fs, collections::HashSet};

fn char_to_ord(el: char) -> u32 {
    let ord = el as u32;
    if ord > 90 {
        return ord - 96;
    } else {
        return ord - 38;
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let lines = input.trim().split("\n");
    let mut part1 = 0;
    let mut part2 = 0;
    let mut groups: Vec<HashSet<char>> = Vec::with_capacity(3);
    for backpack in lines {
        // part1
        let (com1, com2) = backpack.split_at(backpack.len()/2);
        let set1: HashSet<char> = HashSet::from_iter(com1.chars());
        let set2: HashSet<char> = HashSet::from_iter(com2.chars());
        let intersection = set1.intersection(&set2);
        for el in intersection {
            part1 += char_to_ord(*el);
        }
        // part2
        if groups.len() != 3 {
            groups.push(HashSet::from_iter(backpack.chars()));
        } 
        if groups.len() == 3 {
            let inter1: HashSet<&char> = HashSet::from_iter(groups[0].intersection(&groups[1]));
            let inter2: HashSet<&char> = HashSet::from_iter(groups[1].intersection(&groups[2]));
            let intersection = inter1.intersection(&inter2);
            for el in intersection {
                part2 += char_to_ord(**el);
                println!("{} -> {}", el, char_to_ord(**el));
            }
            groups.clear();
        }
    }
    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}
