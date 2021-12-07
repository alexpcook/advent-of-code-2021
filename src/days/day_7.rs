use std::cmp::Ordering;
use std::fs;

pub fn main() {
    println!("### day 7 ###");

    let input = fs::read_to_string("./day_7.txt").expect("could not read input file");
    let crabs: Vec<Crab> = input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|s| s.parse::<u32>().unwrap_or(0))
                .map(Crab::new)
        })
        .flatten()
        .collect();

    // Part 1
    let mut fuel: Vec<u32> = vec![];
    for x in MIN_POSITION..=MAX_POSITION {
        fuel.push(crabs.iter().map(|c| c.fuel_constant(x)).sum());
    }
    println!("part 1: {}", fuel.into_iter().min().unwrap());

    // Part 2
    let mut fuel: Vec<u32> = vec![];
    for x in MIN_POSITION..=MAX_POSITION {
        fuel.push(crabs.iter().map(|c| c.fuel_linear(x)).sum())
    }
    println!("part 2: {}", fuel.into_iter().min().unwrap());
}

/// Minimum horizontal position.
const MIN_POSITION: u32 = 1;

/// Maximum horizontal position.
const MAX_POSITION: u32 = 2000;

/// A crab submarine containing its position.
struct Crab(u32);

impl Crab {
    /// Constructor for crab submarine.
    fn new(position: u32) -> Crab {
        Crab(position)
    }

    /// Calculates the fuel consumption to reach `destination`, assuming a constant rate.
    fn fuel_constant(&self, destination: u32) -> u32 {
        match destination.cmp(&self.0) {
            Ordering::Greater => destination - self.0,
            Ordering::Equal => 0,
            Ordering::Less => self.0 - destination,
        }
    }

    /// Calculates the fuel consumption to reach `destination`, assuming a linear rate.
    fn fuel_linear(&self, destination: u32) -> u32 {
        match destination.cmp(&self.0) {
            Ordering::Greater => (1..(destination - self.0 + 1)).sum(),
            Ordering::Equal => 0,
            Ordering::Less => (1..(self.0 - destination + 1)).sum(),
        }
    }
}
