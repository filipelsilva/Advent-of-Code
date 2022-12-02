use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let rounds = input.trim().split("\n");
    let precedence_abc = vec!["A", "B", "C"];
    let precedence_xyz = vec!["X", "Y", "Z"];
    let mut part1 = 0;
    let mut part2 = 0;
    for round in rounds {
        let mut splits = round.split(" ");
        let (opponent, me) = (splits.next().unwrap(), splits.next().unwrap());
        let index_opponent = precedence_abc.iter().position(|&r| r == opponent).unwrap() as isize;
        let index_me = precedence_xyz.iter().position(|&r| r == me).unwrap() as isize;

        // part1
        part1 += index_me + 1;
        if index_opponent > index_me {
            if index_opponent - index_me != 1 {
                part1 += 6
            }
        } else if index_opponent < index_me {
            if index_me - index_opponent == 1 {
                part1 += 6
            }
        } else {
            part1 += 3;
        }

        // part2
        match index_me {
            // lose
            0 => part2 += ((index_opponent - 1) + 3) % 3 + 1,
            // draw
            1 => part2 += 3 + index_opponent + 1,
            // win
            2 => part2 += 6 + (index_opponent + 1) % 3 + 1,
            // default
            _ => (),
        }
    }
    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}
