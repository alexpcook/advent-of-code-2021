use std::fs;

/// Number of days to simulate for part 1.
const DAYS_PART_1: u32 = 80;

/// Number of days to simulate for part 2.
const DAYS_PART_2: u32 = 256;

/// Threads to solve the problem.
const THREADS: usize = 6;

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
        lanternfish::get_school(&lanternfish, DAYS_PART_1, THREADS)
    );

    // Part 2
    println!(
        "after {} days, there are {} lanternfish",
        DAYS_PART_2,
        lanternfish::get_school(&lanternfish, DAYS_PART_2, THREADS)
    );
}

mod lanternfish {
    use std::cmp::Ordering;
    use std::thread;

    /// The lanternfish reproduction rate.
    const DOUBLING_RATE: usize = 7;

    /// The initial state of new lanternfish.
    const INITIAL_STATE: u32 = 8;

    /// Returns the total number of lanternfish in the school after `days` time,
    /// using `threads` to perform the calculation concurrently.
    pub fn get_school(initial_school: &[u32], days: u32, threads: usize) -> u64 {
        let mut new_fish = 0;
        for (i, chunk) in initial_school.chunks(threads).enumerate() {
            let mut handles = vec![];
            for (j, fish) in chunk.to_owned().into_iter().enumerate() {
                println!("fish {}", threads * i + j);
                handles.push(thread::spawn(move || get_offspring(fish, days)));
            }
            new_fish += handles.into_iter().map(|h| h.join().unwrap()).sum::<u64>();
        }
        initial_school.len() as u64 + new_fish
    }

    /// Returns the total number of offspring for a lanternfish with
    /// `initial_state` after `days` time.
    fn get_offspring(initial_state: u32, days: u32) -> u64 {
        match days.cmp(&initial_state) {
            Ordering::Greater => (initial_state..days)
                .step_by(DOUBLING_RATE)
                .map(|day| 1 + get_offspring(INITIAL_STATE, days - day - 1))
                .sum(),
            _ => 0,
        }
    }
}
