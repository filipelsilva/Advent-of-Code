use std::{fs, collections::HashSet};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let lines = input.trim().split("\n");
    let mut part1: usize = 0;
    let mut part2: usize = 0;
    for line in lines {
        let mut pairs = line.split(",");
        let mut p1 = pairs.next().unwrap().split("-");
        let mut p2 = pairs.next().unwrap().split("-");
        let init1: usize = p1.next().unwrap().parse().unwrap();
        let end1: usize = p1.next().unwrap().parse().unwrap();
        let init2: usize = p2.next().unwrap().parse().unwrap();
        let end2: usize = p2.next().unwrap().parse().unwrap();

        // part1
        if (init1 <= init2 && end1 >= end2) || (init2 <= init1 && end2 >= end1) {
            part1 += 1;
        }

        // part2
        let range1: HashSet<usize> = HashSet::from_iter(init1..end1+1);
        let range2: HashSet<usize> = HashSet::from_iter(init2..end2+1);
        if range1.intersection(&range2).count() != 0 {
            part2 += 1;
        }

        // println!("{} {}", line, part1);
        // println!("{} {}", line, part2);
    }
    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}
