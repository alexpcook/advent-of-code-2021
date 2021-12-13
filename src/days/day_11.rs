use octopus::Pod;
use std::path::Path;

pub fn main() {
    println!("### day 11 ###");

    let input_path = Path::new("./day_11.txt");
    let mut pod_part_1 = Pod::from_file(input_path).expect("could not read input file");
    let pod_part_2 = pod_part_1.clone();

    // Part 1
    println!(
        "part 1: flashes after 100 steps = {}",
        pod_part_1.simulate(100)
    );

    // Part 2
    println!("part 2: steps until sync = {}", pod_part_2.sync());
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
    #[derive(Clone, Debug)]
    pub struct Octopus(u32);

    impl Octopus {
        /// Constructs a new `Octopus` with internal `state`.
        fn new(state: u32) -> Octopus {
            Octopus(state)
        }

        /// Returns the current internal state of the `Octopus`.
        fn state(&self) -> u32 {
            self.0
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

    impl fmt::Display for Octopus {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    /// Models a pod of octopus.
    #[derive(Clone, Debug)]
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

        /// Returns the number of steps until the `Pod` is in sync.
        pub fn sync(mut self) -> u32 {
            let mut step = 0;
            loop {
                step += 1;
                self.simulate(1);
                if self.0.values().all(|o| o.state() == 0) {
                    break;
                }
            }
            step
        }

        /// Increments the state of all `Octopus` in the `Pod`, returning
        /// the number of octopus that flashed during this step.
        fn step(&mut self) -> u32 {
            let mut flashed_positions = vec![];
            for (&position, octopus) in self.0.iter_mut() {
                if octopus.step() == 10 {
                    flashed_positions.push(position);
                }
            }
            self.flash(flashed_positions)
        }

        /// Returns the number of flashes given an initial vector of `flashed_positions`.
        fn flash(&mut self, flashed_positions: Vec<(usize, usize)>) -> u32 {
            let mut flashed = 0;

            for position in flashed_positions {
                flashed += 1;
                let mut more_flashed_positions = vec![];

                for adjacent in self.adjacent_positions(position) {
                    match self.0.get_mut(&adjacent) {
                        Some(octopus) => {
                            if octopus.step() == 10 {
                                more_flashed_positions.push(adjacent);
                            }
                        }
                        None => continue,
                    }
                }

                flashed += self.flash(more_flashed_positions);
            }

            flashed
        }

        /// Returns the adjacent pod positions for `position`.
        fn adjacent_positions(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
            let (x, y) = position;
            let mut adjacent = vec![];

            let (has_left, has_right, has_up, has_down) =
                (x > 0, x < POD_SIZE - 1, y > 0, y < POD_SIZE - 1);

            if has_left {
                adjacent.push((x - 1, y));
                if has_up {
                    adjacent.push((x - 1, y - 1));
                }
                if has_down {
                    adjacent.push((x - 1, y + 1));
                }
            }

            if has_right {
                adjacent.push((x + 1, y));
                if has_up {
                    adjacent.push((x + 1, y - 1));
                }
                if has_down {
                    adjacent.push((x + 1, y + 1));
                }
            }

            if has_up {
                adjacent.push((x, y - 1));
            }

            if has_down {
                adjacent.push((x, y + 1));
            }

            adjacent
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
                    write!(f, "{}", self.0.get(&(i, j)).unwrap_or(&Octopus::new(0)))?;
                }
                writeln!(f)?;
            }
            Ok(())
        }
    }
}
