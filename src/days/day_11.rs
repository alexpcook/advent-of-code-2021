use octopus::Pod;
use std::path::Path;

pub fn main() {
    let input_path = Path::new("./day_11.txt");
    let pod = Pod::from_file(input_path).expect("could not read input file");
}

mod octopus {
    use std::collections::HashMap;
    use std::fs;
    use std::io;
    use std::path::Path;

    /// Width/length of the octopus pod size.
    const POD_SIZE: usize = 10;

    /// Models an octopus.
    #[derive(Debug)]
    pub struct Octopus(u32);

    impl Octopus {
        /// Constructs a new `Octopus` with internal `state`.
        fn new(state: u32) -> Octopus {
            Octopus(state)
        }

        /// Increments the internal state of the `Octopus`, returning the new state value.
        fn step(&mut self) -> u32 {
            self.0 += 1;
            self.0
        }

        /// Resets the internal state of the `Octopus` to `0`.
        fn reset(&mut self) {
            self.0 = 0;
        }
    }

    /// Models a pod of octopus.
    #[derive(Debug)]
    pub struct Pod(HashMap<(usize, usize), u32>);

    impl Pod {
        /// Constructs a pod of octopus from input `path`.
        pub fn from_file(path: &Path) -> io::Result<HashMap<(usize, usize), Octopus>> {
            let contents = fs::read_to_string(path)?;
            let mut octopus = HashMap::with_capacity(POD_SIZE * POD_SIZE);

            for (i, line) in contents.lines().enumerate() {
                for (j, digit) in line.chars().enumerate() {
                    octopus.insert(
                        (i, j),
                        Octopus::new(digit.to_digit(10).ok_or_else(|| {
                            io::Error::new(
                                io::ErrorKind::Other,
                                format!("invalid octopus state: {}", digit),
                            )
                        })?),
                    );
                }
            }

            Ok(octopus)
        }
    }
}
