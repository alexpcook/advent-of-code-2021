use std::fs;

/// Number of days to simulate for part 1.
const DAYS_PART_1: u32 = 80;

/// Number of days to simulate for part 2.
const DAYS_PART_2: u32 = 256;

pub fn main() {
    let contents = fs::read_to_string("./day_6.txt").expect("could not read input file");
    let mut lanternfish: Vec<Lanternfish> = contents
        .lines()
        .map(|l| {
            l.split(',')
                .map(|s| s.parse::<u32>().unwrap_or(0))
                .map(Lanternfish::new)
        })
        .flatten()
        .collect();

    // Part 1
    for day in 1..=DAYS_PART_1 {
        let mut new_fish = vec![];

        for fish in lanternfish.iter_mut() {
            if let Some(f) = fish.age() {
                new_fish.push(f);
            }
        }

        for fish in new_fish {
            lanternfish.push(fish);
        }

        println!(
            "on day {}, there are {} lanternfish",
            day,
            lanternfish.len()
        );
    }
}

/// Models a lanternfish.
#[derive(Debug)]
struct Lanternfish(u32);

impl Default for Lanternfish {
    fn default() -> Lanternfish {
        Lanternfish(Self::TIMER_NEW_VALUE)
    }
}

impl Lanternfish {
    /// Reset timer value for a lanternfish after reaching 0.
    const TIMER_RESET_VALUE: u32 = 6;

    /// Initial timer value for a new lanternfish.
    const TIMER_NEW_VALUE: u32 = 8;

    /// Creates a new lanternfish with `timer` internal state.
    fn new(timer: u32) -> Lanternfish {
        Lanternfish(timer)
    }

    /// Ages a lanternfish by one day. Returns a new lanternfish if one was created.
    fn age(&mut self) -> Option<Lanternfish> {
        match self.0 {
            0 => {
                self.0 = Self::TIMER_RESET_VALUE;
                Some(Self::default())
            }
            _ => {
                self.0 -= 1;
                None
            }
        }
    }
}
