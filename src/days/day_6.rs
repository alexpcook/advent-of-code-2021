use std::fs;

/// Number of days to simulate for part 1.
const DAYS_PART_1: u32 = 80;

/// Number of days to simulate for part 2.
const DAYS_PART_2: u32 = 256;

pub fn main() {
    let contents = fs::read_to_string("./day_6.txt").expect("could not read input file");
    let lanternfish: Vec<u32> = contents
        .lines()
        .map(|l| l.split(',').map(|s| s.parse::<u32>().unwrap_or(0)))
        .flatten()
        .collect();

    // Part 1
    println!(
        "after {} days, there are {} lanternfish",
        DAYS_PART_1,
        lanternfish::get_school(&lanternfish, DAYS_PART_1)
    );

    // Part 2
    println!(
        "after {} days, there are {} lanternfish",
        DAYS_PART_2,
        lanternfish::get_school(&lanternfish, DAYS_PART_2)
    );
}

mod lanternfish {
    use std::cmp::Ordering;

    /// The lanternfish reproduction rate.
    const DOUBLING_RATE: usize = 7;

    /// The initial state of new lanternfish.
    const INITIAL_STATE: u32 = 8;

    /// Returns the total number of lanternfish (including the `initial_school`)
    /// in the school after `days` time.
    pub fn get_school(initial_school: &[u32], days: u32) -> u32 {
        let mut new_fish = 0;
        for (i, fish) in initial_school.iter().enumerate() {
            println!("fish {}", i);
            new_fish += get_offspring(*fish, days);
        }
        initial_school.len() as u32 + new_fish
    }

    /// Returns the total number of offspring for a lanternfish with
    /// `initial_state` after `days` time.
    fn get_offspring(initial_state: u32, days: u32) -> u32 {
        match days.cmp(&initial_state) {
            Ordering::Greater => (initial_state..days)
                .step_by(DOUBLING_RATE)
                .map(|day| 1 + get_offspring(INITIAL_STATE, days - day - 1))
                .sum(),
            _ => 0,
        }
    }
}
