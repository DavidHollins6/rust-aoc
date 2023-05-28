use std::collections::HashMap;
use std::fs;

fn main() {
    let outcomes_part_1: HashMap<&str, usize> = HashMap::from([
        ("AX", 4),
        ("AY", 8),
        ("AZ", 3),
        ("BX", 1),
        ("BY", 5),
        ("BZ", 9),
        ("CX", 7),
        ("CY", 2),
        ("CZ", 6),
    ]);

    let outcomes_part_2: HashMap<&str, usize> = HashMap::from([
        ("AX", 3),
        ("AY", 4),
        ("AZ", 8),
        ("BX", 1),
        ("BY", 5),
        ("BZ", 9),
        ("CX", 2),
        ("CY", 6),
        ("CZ", 7),
    ]);

    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut score_part_1: usize = 0;
    let mut score_part_2: usize = 0;

    for line in contents.lines() {
        let matchup = line.replace(" ", "");
        let matchup_score = match outcomes_part_1.get::<str>(&matchup) {
            Some(res) => res,
            None => &0,
        };
        score_part_1 += matchup_score;

        let matchup_score_part_2 = match outcomes_part_2.get::<str>(&matchup) {
            Some(res) => res,
            None => &0,
        };
        score_part_2 += matchup_score_part_2;
    }
    println!("part 1: {:?}", score_part_1);
    println!("part 2: {:?}", score_part_2);
}
