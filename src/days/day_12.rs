use cave::Caves;
use std::path::Path;

pub fn main() {
    println!("### day 12 ###");

    let input = Path::new("./day_12.txt");
    let caves = Caves::from_file(input).expect("could not read input file");

    // Part 1
    println!("part 1: number of paths = {}", caves.paths(false));

    // Part 2
    println!("part 2: number of paths = {}", caves.paths(true));
}

mod cave {
    use std::collections::HashMap;
    use std::fs;
    use std::io;
    use std::path::Path;

    /// The type of cave.
    #[derive(Debug)]
    enum CaveType {
        Big,
        Small,
        Start,
        End,
    }

    /// Models a cave.
    #[derive(Debug)]
    struct Cave {
        cave_type: CaveType,
        connections: Vec<String>,
    }

    impl TryFrom<&str> for Cave {
        type Error = io::Error;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            match value {
                "start" => Ok(Cave {
                    cave_type: CaveType::Start,
                    connections: vec![],
                }),
                "end" => Ok(Cave {
                    cave_type: CaveType::End,
                    connections: vec![],
                }),
                value if value.chars().all(|c| matches!(c, 'a'..='z')) => Ok(Cave {
                    cave_type: CaveType::Small,
                    connections: vec![],
                }),
                value if value.chars().all(|c| matches!(c, 'A'..='Z')) => Ok(Cave {
                    cave_type: CaveType::Big,
                    connections: vec![],
                }),
                _ => Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("cannot parse Cave from string slice '{}'", value),
                )),
            }
        }
    }

    /// Models a network of caves.
    #[derive(Debug)]
    pub struct Caves(HashMap<String, Cave>);

    impl Caves {
        /// Parses an instance of `Caves` from the input file at `path`.
        pub fn from_file(path: &Path) -> io::Result<Caves> {
            let mut caves = Caves(HashMap::new());
            let contents = fs::read_to_string(path)?;

            for line in contents.lines() {
                let cave_pair: Vec<_> = line.splitn(2, '-').collect();
                let &cave_1 = cave_pair.get(0).unwrap_or(&"");
                let &cave_2 = cave_pair.get(1).unwrap_or(&"");

                caves
                    .0
                    .entry(cave_1.to_owned())
                    .or_insert(Cave::try_from(cave_1)?)
                    .connections
                    .push(cave_2.to_owned());
                caves
                    .0
                    .entry(cave_2.to_owned())
                    .or_insert(Cave::try_from(cave_2)?)
                    .connections
                    .push(cave_1.to_owned());
            }

            Ok(caves)
        }

        /// Returns the number of unique paths from the cave start to end.
        /// Big caves can be visited any number of times. Small caves can be visited once,
        /// unless `allow_one_small_cave_twice` is true, in which case a single small cave
        /// in the network can be optionally traversed more than once. The start cave cannot
        /// be revisited, and the path ends immediately upon reaching the end cave.
        pub fn paths(&self, allow_one_small_cave_twice: bool) -> u32 {
            match self.0.get("start") {
                None => 0,
                Some(cave) => self.traverse(
                    &cave.connections,
                    HashMap::new(),
                    allow_one_small_cave_twice,
                ),
            }
        }

        /// Traverses a cave's `connections`, keeping track of small caves that have been `visited`.
        fn traverse(
            &self,
            connections: &[String],
            visited: HashMap<String, u32>,
            allow_one_small_cave_twice: bool,
        ) -> u32 {
            let mut paths = 0;

            for connection in connections {
                // Each connection is a potential unique path, so clone the HashMap of visited small caves
                let mut visited = visited.clone();

                if let Some(cave) = self.0.get(connection) {
                    match cave.cave_type {
                        CaveType::Big => {
                            paths += self.traverse(
                                &cave.connections,
                                visited,
                                allow_one_small_cave_twice,
                            );
                        }
                        CaveType::Small => {
                            if let Some(x) = visited.get_mut(connection) {
                                // Invalid path if not allowing double traversal of a single small cave
                                if !allow_one_small_cave_twice {
                                    continue;
                                }

                                *x += 1;

                                // Invalid path if small cave traversed more than two times, or if more than
                                // one small cave traversed two times
                                if *x > 2 || visited.iter().filter(|(_, &v)| v > 1).count() > 1 {
                                    continue;
                                }
                            } else {
                                visited.insert(connection.clone(), 1);
                            }

                            paths += self.traverse(
                                &cave.connections,
                                visited,
                                allow_one_small_cave_twice,
                            );
                        }
                        CaveType::Start => continue,
                        CaveType::End => paths += 1,
                    }
                }
            }

            paths
        }
    }
}
