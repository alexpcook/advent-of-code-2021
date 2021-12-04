use diagnostic::BinaryDiagnostic;

pub fn main() {
    println!("### day 3 ###");

    let binary_diagnostic =
        BinaryDiagnostic::from_file("./day_3.txt").expect("could not read input file");

    // Part 1
    println!("part 1: {}", binary_diagnostic.power_consumption());

    // Part 2
    println!("part 2: {}", binary_diagnostic.life_support_rating());
}

mod diagnostic {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    /// The number of binary digits in one line of diagnostic input data.
    static BINARY_LINE_LENGTH: usize = 12;

    /// The binary diagnostic info for the submarine.
    pub struct BinaryDiagnostic {
        data: Vec<String>,
    }

    impl BinaryDiagnostic {
        /// Initializes the submarine's binary diagnostic readings from an input file at `path`.
        pub fn from_file(path: &str) -> io::Result<BinaryDiagnostic> {
            let data = io::BufReader::new(File::open(Path::new(path))?)
                .lines()
                .map(|l| l.unwrap_or_default())
                .collect::<Vec<_>>();
            Ok(BinaryDiagnostic { data })
        }

        /// Calculates the power consumption of the submarine.
        pub fn power_consumption(&self) -> u32 {
            let mut gamma_rate = String::with_capacity(BINARY_LINE_LENGTH);
            let mut epsilon_rate = String::with_capacity(BINARY_LINE_LENGTH);

            for i in 0..BINARY_LINE_LENGTH {
                let (g, e) = match Self::most_common_bit(&self.data, i) {
                    Some(x) if x == 1 => ('1', '0'),
                    Some(x) if x == 0 => ('0', '1'),
                    _ => continue,
                };
                gamma_rate.push(g);
                epsilon_rate.push(e);
            }

            let gamma_rate = u32::from_str_radix(&gamma_rate, 2).unwrap_or(0);
            let epsilon_rate = u32::from_str_radix(&epsilon_rate, 2).unwrap_or(0);

            gamma_rate * epsilon_rate
        }

        /// Calculates the life support rating of the submarine.
        pub fn life_support_rating(&self) -> u32 {
            let oxygen_generator_rating = self.get_rating(true);
            let co2_scrubber_rating = self.get_rating(false);
            oxygen_generator_rating * co2_scrubber_rating
        }

        /// Calculates the most common bit from `data` at `position`.
        /// Returns `Some(1)` if `1` is most common, `Some(0)` if `0` is most common,
        /// and `None` if `1` and `0` are equally common.
        /// # Panics
        /// Panics if `position >= BINARY_LINE_LENGTH`.
        fn most_common_bit(data: &[String], position: usize) -> Option<u8> {
            if position >= BINARY_LINE_LENGTH {
                panic!(
                    "cannot equal or exceed BINARY_LINE_LENGTH {}",
                    BINARY_LINE_LENGTH
                );
            }

            let result = data
                .iter()
                .flat_map(|s| s.chars().collect::<Vec<_>>())
                .skip(position)
                .step_by(BINARY_LINE_LENGTH)
                .fold(0, |mut acc, c| {
                    if c == '1' {
                        acc += 1;
                    } else {
                        acc -= 1;
                    }
                    acc
                });

            match result {
                0 => None,
                1.. => Some(1),
                _ => Some(0),
            }
        }

        /// Gets a rating for the submarine. If `use_most_common` is `true`, returns
        /// the oxygen generator rating, else returns the CO2 scrubber rating.
        fn get_rating(&self, use_most_common: bool) -> u32 {
            let mut data = self.data.clone();

            for i in 0..BINARY_LINE_LENGTH {
                let keep_bit = match Self::most_common_bit(&data, i) {
                    Some(x) if x == 1 => {
                        if use_most_common {
                            '1'
                        } else {
                            '0'
                        }
                    }
                    Some(x) if x == 0 => {
                        if use_most_common {
                            '0'
                        } else {
                            '1'
                        }
                    }
                    None => {
                        if use_most_common {
                            '1'
                        } else {
                            '0'
                        }
                    }
                    _ => continue,
                };

                data = data
                    .into_iter()
                    .filter(|s| matches!(s.chars().nth(i), Some(c) if c == keep_bit))
                    .collect::<Vec<_>>();

                if data.len() == 1 {
                    break;
                }
            }

            u32::from_str_radix(&data[0], 2).unwrap_or(0)
        }
    }
}
