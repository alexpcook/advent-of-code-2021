use octopus::Pod;
use std::path::Path;

pub fn main() {
    let input_path = Path::new("./day_11.txt");
    let mut pod = Pod::from_file(input_path).expect("could not read input file");

    // Part 1
    println!("part 1: flashes after 100 steps = {}", pod.simulate(100));
}

mod octopus {
    use std::collections::HashMap;
    use std::fmt;
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

        /// Resets the internal state of the `Octopus` to `0` if it flashed.
        fn reset(&mut self) {
            if self.0 > 9 {
                self.0 = 0;
            }
        }
    }

    /// Models a pod of octopus.
    #[derive(Debug)]
    pub struct Pod(HashMap<(usize, usize), Octopus>);

    impl Pod {
        /// Constructs a `Pod` of octopus from input `path`.
        pub fn from_file(path: &Path) -> io::Result<Pod> {
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

            Ok(Pod(octopus))
        }

        /// Simulates a `Pod` for a given number of `steps`.
        pub fn simulate(&mut self, steps: u32) -> u32 {
            let mut flashed = 0;
            for _ in 0..steps {
                flashed += self.step();
                self.reset();
            }
            flashed
        }

        /// Increments the state of all `Octopus` in the `Pod`.
        fn step(&mut self) -> u32 {
            let mut flashed = 0;
            for octopus in self.0.values_mut() {
                let new_state = octopus.step();
                if new_state == 10 {
                    flashed += 1;
                }
            }
            flashed
        }

        /// Resets all `Octopus` in the `Pod` that flashed.
        fn reset(&mut self) {
            for octopus in self.0.values_mut() {
                octopus.reset();
            }
        }
    }

    impl fmt::Display for Pod {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for i in 0..POD_SIZE {
                for j in 0..POD_SIZE {
                    write!(f, "{}", self.0.get(&(i, j)).unwrap_or(&Octopus::new(0)).0)?;
                }
                writeln!(f)?;
            }
            Ok(())
        }
    }
}
