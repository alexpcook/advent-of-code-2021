use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    println!("### day 1 ###");

    let depth = Depth::from_file("./day_1.txt").expect("could not read input file");

    // Part 1
    println!(
        "part 1: {} {}",
        depth.increases_v1(1),
        depth.increases_v2(1),
    );

    // Part 2
    println!(
        "part 2: {} {}",
        depth.increases_v1(3),
        depth.increases_v2(3),
    );
}

/// Represents a list of depths as a `Vec<i32>`.
struct Depth(Vec<i32>);

impl Depth {
    /// Reads a list of depths from the file `path`.
    fn from_file(path: &str) -> io::Result<Depth> {
        let depths = io::BufReader::new(File::open(Path::new(path))?)
            .lines()
            .map(|l| {
                l.and_then(|s| {
                    s.parse::<i32>().map_err(|e| {
                        io::Error::new(io::ErrorKind::Other, format!("invalid depth: {}", e))
                    })
                })
            })
            .collect::<io::Result<Vec<i32>>>()?;
        Ok(Depth(depths))
    }

    /// Returns the number of times the depth increases between `offset` number of readings.
    fn increases_v1(&self, offset: usize) -> usize {
        self.0
            .iter()
            .enumerate()
            .filter(|(i, &curr)| matches!(self.0.get(i + offset), Some(&next) if next > curr))
            .count()
    }

    /// Returns the number of times the depth increases between `offset` number of readings.
    fn increases_v2(&self, offset: usize) -> usize {
        self.0
            .iter()
            .skip(offset)
            .zip(self.0.iter())
            .filter(|(&next, &curr)| next > curr)
            .count()
    }
}
