use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut parts = input.trim_end().split("\n\n");
    let starting = parts.next().unwrap();

    // parse the initial arrangement
    let crates_tmp = starting.split("\n");
    // FIXME this clone is stupid
    let mut crates: Vec<Vec<String>> = Vec::with_capacity(crates_tmp.clone().count()-1);
    for line in crates_tmp {
        let temp = line.as_bytes()
            .chunks(4)
            .map(std::str::from_utf8)
            .map(|part| {
                Ok::<String, String>(part.unwrap()
                    .replace(" ", "")
                    .replace("]", "")
                    .replace("[", ""))
            })
            .collect::<Result<Vec<String>, _>>()
            .unwrap();

        for i in 0..crates.len() {
            // FIXME clone
            crates[i].push(temp[i].clone());
        }
    }
    println!("{:?}", crates);

    // parse the moves
    let mut moves = Vec::new();
    let moves_tmp = parts.next().unwrap().split("\n");
    moves_tmp.for_each(|part: &str| {
        let mut single_move = Vec::new();
        part.replace("move ", "").replace("from ", "").replace(" to", "")
            .split(" ").for_each(|part: &str| {
                single_move.push(part.parse::<usize>().unwrap());
            });
        moves.push(single_move);
    });
}
