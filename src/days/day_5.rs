use hydrothermal::Vents;

pub fn main() {
    println!("### day 5 ###");

    let vents = Vents::from_file("./day_5.txt").expect("could not read input file");

    // Part 1
    println!("part 1: {}", vents.overlaps(false));

    // Part 2
    println!("part 2: {}", vents.overlaps(true));
}

mod hydrothermal {
    use std::collections::HashMap;
    use std::fs;
    use std::io;

    /// The size of the submarine's hydrothermal vent map.
    const MAP_SIZE: usize = 1000 * 1000;

    /// A point on the map.
    #[derive(Debug)]
    struct Point {
        x: usize,
        y: usize,
    }

    /// A hydrothermal vent.
    #[derive(Debug)]
    struct Vent {
        p1: Point,
        p2: Point,
    }

    /// The set of all hydrothermal vents on the submarine's map.
    #[derive(Debug)]
    pub struct Vents(Vec<Vent>);

    impl Vents {
        /// Parses hydrothermal vents from the file at `path`.
        pub fn from_file(path: &str) -> io::Result<Vents> {
            let mut vents = Vents(vec![]);

            let contents = fs::read_to_string(path)?;
            for line in contents.lines() {
                let pairs = line.splitn(2, " -> ").collect::<Vec<&str>>();
                if pairs.len() != 2 {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("want two pairs, got {}", pairs.len()),
                    ));
                }

                let p1_coordinates = pairs[0]
                    .splitn(2, ',')
                    .map(|s| s.parse::<usize>().unwrap_or(0))
                    .collect::<Vec<usize>>();
                if p1_coordinates.len() != 2 {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("want two p1 coordinates, got {}", p1_coordinates.len()),
                    ));
                }

                let p2_coordinates = pairs[1]
                    .splitn(2, ',')
                    .map(|s| s.parse::<usize>().unwrap_or(0))
                    .collect::<Vec<usize>>();
                if p2_coordinates.len() != 2 {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("want two p2 coordinates, got {}", p2_coordinates.len()),
                    ));
                }

                vents.0.push(Vent {
                    p1: Point {
                        x: p1_coordinates[0],
                        y: p1_coordinates[1],
                    },
                    p2: Point {
                        x: p2_coordinates[0],
                        y: p2_coordinates[1],
                    },
                })
            }

            Ok(vents)
        }

        /// Returns the number of points on the map where at least two vents overlap.
        pub fn overlaps(&self, consider_diagonals: bool) -> usize {
            let mut map: HashMap<(usize, usize), u32> = HashMap::with_capacity(MAP_SIZE);
            for vent in &self.0 {
                if vent.p1.x == vent.p2.x {
                    let range = if vent.p1.y > vent.p2.y {
                        vent.p2.y..vent.p1.y + 1
                    } else {
                        vent.p1.y..vent.p2.y + 1
                    };
                    for i in range {
                        let key = (vent.p1.x, i);
                        if let Some(n) = map.get_mut(&key) {
                            *n += 1;
                        } else {
                            map.insert(key, 1);
                        }
                    }
                } else if vent.p1.y == vent.p2.y {
                    let range = if vent.p1.x > vent.p2.x {
                        vent.p2.x..vent.p1.x + 1
                    } else {
                        vent.p1.x..vent.p2.x + 1
                    };
                    for i in range {
                        let key = (i, vent.p1.y);
                        if let Some(n) = map.get_mut(&key) {
                            *n += 1;
                        } else {
                            map.insert(key, 1);
                        }
                    }
                } else if consider_diagonals {
                    let (x_range, y_range, reverse_y) = if vent.p1.x > vent.p2.x {
                        if vent.p1.y > vent.p2.y {
                            (vent.p2.x..vent.p1.x + 1, vent.p2.y..vent.p1.y + 1, false)
                        } else {
                            (vent.p2.x..vent.p1.x + 1, vent.p1.y..vent.p2.y + 1, true)
                        }
                    } else if vent.p1.y > vent.p2.y {
                        (vent.p1.x..vent.p2.x + 1, vent.p2.y..vent.p1.y + 1, true)
                    } else {
                        (vent.p1.x..vent.p2.x + 1, vent.p1.y..vent.p2.y + 1, false)
                    };
                    if reverse_y {
                        for key in x_range.zip(y_range.rev()) {
                            if let Some(n) = map.get_mut(&key) {
                                *n += 1;
                            } else {
                                map.insert(key, 1);
                            }
                        }
                    } else {
                        for key in x_range.zip(y_range) {
                            if let Some(n) = map.get_mut(&key) {
                                *n += 1;
                            } else {
                                map.insert(key, 1);
                            }
                        }
                    }
                }
            }

            map.values().filter(|v| v > &&1).count()
        }
    }
}
