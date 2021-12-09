use std::collections::{HashMap, HashSet};
use std::fs;

pub fn main() {
    println!("### day 8 ###");

    let input = fs::read_to_string("./day_8.txt").expect("could not read input file");
    let lines: Vec<_> = input
        .lines()
        .map(|line| line.split(" | "))
        .map(|mut pieces| {
            let mut patterns = vec![];
            for pattern in pieces.next().unwrap().split(' ') {
                let mut p = pattern.chars().collect::<Vec<_>>();
                p.sort_by(|a, b| b.cmp(a));
                patterns.push(p.into_iter().fold(String::with_capacity(7), |mut s, c| {
                    s.push(c);
                    s
                }));
            }

            let mut digits = vec![];
            for digit in pieces.next().unwrap().split(' ') {
                let mut d = digit.chars().collect::<Vec<_>>();
                d.sort_by(|a, b| b.cmp(a));
                digits.push(d.into_iter().fold(String::with_capacity(7), |mut s, c| {
                    s.push(c);
                    s
                }));
            }

            (patterns, digits)
        })
        .collect();

    // Part 1
    println!("digits 1, 4, 7, and 8 appear {} times", part_1(&lines));

    // Part 2
    println!("sum of all numbers is {}", part_2(&lines));
}

/// Solution for part 1.
fn part_1(lines: &[(Vec<String>, Vec<String>)]) -> u32 {
    lines
        .iter()
        .flat_map(|line| &line.1)
        .filter(|digit| matches!(digit.len(), 2 | 4 | 3 | 7)) // digit is 1, 4, 7, or 8 (based on number of segments)
        .fold(0, |count, _| count.checked_add(1).unwrap())
}

/// Solution for part 2.
fn part_2(lines: &[(Vec<String>, Vec<String>)]) -> u32 {
    let mut sum = 0;

    for line in lines {
        let mut patterns_to_chars = HashMap::with_capacity(10);
        let mut numbers_to_patterns = HashMap::with_capacity(10);

        // Find 1, 4, 7, 8
        for pattern in &line.0 {
            match pattern.len() {
                // 1 has two segments
                2 => {
                    patterns_to_chars.insert(pattern, '1');
                    numbers_to_patterns.insert(1, pattern);
                }
                // 4 has four segments
                4 => {
                    patterns_to_chars.insert(pattern, '4');
                    numbers_to_patterns.insert(4, pattern);
                }
                // 7 has three segments
                3 => {
                    patterns_to_chars.insert(pattern, '7');
                    numbers_to_patterns.insert(7, pattern);
                }
                // 8 has seven segments
                7 => {
                    patterns_to_chars.insert(pattern, '8');
                    numbers_to_patterns.insert(8, pattern);
                }
                _ => continue,
            }
        }

        let one_chars: HashSet<_> = numbers_to_patterns.get(&1).unwrap().chars().collect();
        let four_chars: HashSet<_> = numbers_to_patterns.get(&4).unwrap().chars().collect();

        // Find remaining numbers
        for pattern in &line.0 {
            match pattern.len() {
                5 => {
                    if pattern
                        .chars()
                        .collect::<HashSet<_>>()
                        .intersection(&one_chars)
                        .count()
                        == 2
                    {
                        // 3 is the only number with five segments that contains 1
                        patterns_to_chars.insert(pattern, '3');
                        numbers_to_patterns.insert(3, pattern);
                    } else if pattern
                        .chars()
                        .collect::<HashSet<_>>()
                        .intersection(&four_chars)
                        .count()
                        == 2
                    {
                        // 2 is the only number with five segments that shares half of the
                        // characters in its pattern with 4
                        patterns_to_chars.insert(pattern, '2');
                        numbers_to_patterns.insert(2, pattern);
                    } else {
                        // The only other possibility is 5
                        patterns_to_chars.insert(pattern, '5');
                        numbers_to_patterns.insert(5, pattern);
                    }
                }
                6 => {
                    if pattern
                        .chars()
                        .collect::<HashSet<_>>()
                        .intersection(&four_chars)
                        .count()
                        == 4
                    {
                        // 9 is the only number with six segments that contains 4
                        patterns_to_chars.insert(pattern, '9');
                        numbers_to_patterns.insert(9, pattern);
                    } else if pattern
                        .chars()
                        .collect::<HashSet<_>>()
                        .intersection(&one_chars)
                        .count()
                        == 2
                    {
                        // Else 0 contains 1
                        patterns_to_chars.insert(pattern, '0');
                        numbers_to_patterns.insert(0, pattern);
                    } else {
                        // Else 6
                        patterns_to_chars.insert(pattern, '6');
                        numbers_to_patterns.insert(6, pattern);
                    }
                }
                _ => continue,
            }
        }

        // Parse digits
        let mut number = String::with_capacity(4);
        for digit in &line.1 {
            match patterns_to_chars.get(digit) {
                Some(&d) => number.push(d),
                None => panic!("found unknown pattern: {}", digit),
            }
        }

        // Add to total
        sum += number.parse::<u32>().unwrap();
    }

    sum
}
