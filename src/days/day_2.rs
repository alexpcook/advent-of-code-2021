use direction::Directions;

pub fn main() {
    println!("### day 2 ###");

    let directions = Directions::from_file("./day_2.txt").expect("could not read input file");

    // Part 1
    println!("part 1: {}", directions.position(false));

    // Part 2
    println!("part 2: {}", directions.position(true));
}

mod direction {
    use std::fmt;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    /// Command for the submarine.
    enum Command {
        /// Moves forward.
        Forward,
        /// Increases depth/aim.
        Down,
        /// Decreases depth/aim.
        Up,
    }

    /// Direction for the submarine.
    struct Direction {
        /// Command for the direction.
        command: Command,
        /// Distance for the command.
        distance: u32,
    }

    impl TryFrom<String> for Direction {
        type Error = &'static str;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            let (command, distance) = value.split_once(" ").ok_or("invalid direction")?;

            let command = match command {
                "forward" => Command::Forward,
                "down" => Command::Down,
                "up" => Command::Up,
                _ => return Err("invalid command"),
            };

            let distance = distance.parse::<u32>().map_err(|_| "invalid distance")?;

            Ok(Direction { command, distance })
        }
    }

    /// A set of directions for the submarine.
    pub struct Directions(Vec<Direction>);

    /// Represents the position of the submarine.
    pub struct Position {
        /// Horizontal distance of the submarine.
        pub distance: u32,
        /// Depth of the submarine.
        pub depth: u32,
        /// Aim of the submarine.
        aim: u32,
    }

    impl Default for Position {
        fn default() -> Position {
            Position {
                distance: 0,
                depth: 0,
                aim: 0,
            }
        }
    }

    impl fmt::Display for Position {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "(distance: {}, depth: {})", self.distance, self.depth)
        }
    }

    impl Directions {
        /// Parses a set of submarine directions from the file at `path`.
        pub fn from_file(path: &str) -> io::Result<Directions> {
            let directions = io::BufReader::new(File::open(Path::new(path))?)
                .lines()
                .map(|l| {
                    l.and_then(|s| {
                        Direction::try_from(s).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
                    })
                })
                .collect::<io::Result<Vec<Direction>>>()?;
            Ok(Directions(directions))
        }

        /// Calculates the position of the submarine based on the set of directions.
        /// The `use_aim` flag determines whether to factor in the aim as part of the guidance system.
        pub fn position(&self, use_aim: bool) -> Position {
            self.0.iter().fold(
                Position::default(),
                |mut pos,
                 Direction {
                     command,
                     distance: x,
                 }| match command {
                    Command::Forward => {
                        pos.distance += x;
                        if use_aim {
                            pos.depth += pos.aim * x;
                        }
                        pos
                    }
                    Command::Down => {
                        if use_aim {
                            pos.aim += x;
                        } else {
                            pos.depth += x;
                        }
                        pos
                    }
                    Command::Up => {
                        if use_aim {
                            pos.aim -= x;
                        } else {
                            pos.depth -= x;
                        }
                        pos
                    }
                },
            )
        }
    }
}
