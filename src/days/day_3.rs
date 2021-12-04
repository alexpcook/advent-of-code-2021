use diagnostic::BinaryDiagnostic;

pub fn main() {
    println!("### day 3 ###");

    // Part 1
    println!(
        "part 1: {}",
        BinaryDiagnostic::power_consumption("./day_3.txt").expect("could not read input file")
    );

    // Part 2
    println!("part 2: {}", 0);
}

mod diagnostic {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    /// The number of binary digits in one line of input.
    static BINARY_LINE_LENGTH: usize = 12;

    /// The binary diagnostic info for the submarine.
    pub struct BinaryDiagnostic;

    impl BinaryDiagnostic {
        /// Calculate the power consumption of the submarine from an input file at `path`.
        pub fn power_consumption(path: &str) -> io::Result<u32> {
            let (gamma_rate, epsilon_rate) = io::BufReader::new(File::open(Path::new(path))?)
                .lines()
                .map(|l| l.unwrap_or_default())
                // Flatten the input file into a vector of characters
                .flat_map(|s| s.chars().collect::<Vec<_>>())
                .enumerate()
                // Fold the vector of characters into another vector with BINARY_LINE_LENGTH
                // number of entries (one for each column of data we need to read)
                // Increment each column's entry by 1 if '1', else decrement by 1 if '0'
                .fold(vec![0; BINARY_LINE_LENGTH], |mut res, (i, c)| {
                    res[i % BINARY_LINE_LENGTH] += if c == '1' { 1 } else { -1 };
                    res
                })
                .into_iter()
                // Fold the vector into a tuple for the gamma and epsilon rates
                .fold(
                    (
                        String::with_capacity(BINARY_LINE_LENGTH),
                        String::with_capacity(BINARY_LINE_LENGTH),
                    ),
                    |(mut gamma, mut epsilon), i| {
                        let (g, e) = match i {
                            // 1 is most common entry in the column
                            1.. => ('1', '0'),
                            // 0 is the most common entry in the column
                            _ => ('0', '1'),
                        };
                        gamma.push(g);
                        epsilon.push(e);
                        (gamma, epsilon)
                    },
                );

            // Parse the gamma and epsilon strings into binary numbers
            let gamma_rate = u32::from_str_radix(&gamma_rate, 2).unwrap_or(0);
            let epsilon_rate = u32::from_str_radix(&epsilon_rate, 2).unwrap_or(0);

            Ok(gamma_rate * epsilon_rate)
        }
    }
}
