use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut current_elf_total = 0;
    let mut elves: Vec<usize> = vec![];

    for line in contents.lines() {
        if line == "" {
            elves.push(current_elf_total);

            current_elf_total = 0;
        } else {
            let calories = match line.parse::<usize>() {
                Ok(cal) => cal,
                Err(err) => panic!("Not a number in this file!!! {:?}", err),
            };
            current_elf_total += calories;
        }
    }
    elves.sort_by(|a, b| b.cmp(a));

    println!(
        "{:?}",
        elves
            .into_iter()
            .take(3)
            .sum::<usize>()
    );
}
