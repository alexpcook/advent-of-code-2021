use std::fs;

pub fn main() {
    println!("### day 8 ###");

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

    // Part 2
    println!("sum of all numbers is {}", part_2(&lines));
}

/// Solution for part 1.
fn part_1(input: &[(Vec<&str>, Vec<&str>)]) -> u32 {
    input
        .iter()
        .flat_map(|line| &line.1)
        .filter(|digit| matches!(digit.len(), 2 | 4 | 3 | 7)) // digit is 1, 4, 7, or 8
        .fold(0, |count, _| count.checked_add(1).unwrap())
}

// Solution for part 2.
fn part_2(input: &[(Vec<&str>, Vec<&str>)]) -> u32 {
    0
}
