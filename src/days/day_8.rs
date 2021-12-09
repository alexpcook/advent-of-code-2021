use std::fs;

pub fn main() {
    let input = fs::read_to_string("./day_8.txt").expect("could not read input file");
    let lines = input
        .lines()
        .map(|line| line.split(" | "))
        .map(|mut pieces| {
            (
                pieces.next().unwrap().split(' ').collect::<Vec<_>>(),
                pieces.next().unwrap().split(' ').collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    // Part 1
    println!("digits 1, 4, 7, and 8 appear {} times", part_1(&lines));
}

/// Solution for part 1.
fn part_1(input: &[(Vec<&str>, Vec<&str>)]) -> u32 {
    let mut count = 0;
    for line in input {
        for digit in &line.1 {
            match digit.len() {
                // 1 = 2 parts, 4 = 4 parts, 7 = 3 parts, 8 = 7 parts
                2 | 4 | 3 | 7 => count += 1,
                _ => {}
            }
        }
    }
    count
}
