use hydrothermal::Vents;

pub fn main() {
    println!("### day 5 ###");

    let vents = Vents::from_file("./day_5.txt").expect("could not read input file");

    // Part 1
    println!("part 1: {}", vents.overlapping(false));

    // Part 2
    println!("part 2: {}", vents.overlapping(true));
}

mod hydrothermal {
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::fs;
    use std::io;
    use std::iter;

    /// The length/width of the submarine's hydrothermal vent map.
    const MAP_SIZE: usize = 1000;

    /// A point on the map.
    #[derive(Debug)]
    struct Point {
        x: usize,
        y: usize,
    }

    impl TryFrom<&str> for Point {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let coordinates: Vec<usize> = value
                .splitn(2, ',')
                .map(|s| s.parse::<usize>().unwrap_or(0))
                .collect();
            let (x, y) = match coordinates.get(0..2) {
                Some(rng) => (rng[0], rng[1]),
                None => return Err("want two integers to construct point"),
            };
            Ok(Point { x, y })
        }
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

                let (p1, p2) = match pairs.get(0..2) {
                    Some(rng) => (
                        Point::try_from(rng[0])
                            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?,
                        Point::try_from(rng[1])
                            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?,
                    ),
                    None => {
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            "want two points to construct vent",
                        ))
                    }
                };

                vents.0.push(Vent { p1, p2 })
            }

            Ok(vents)
        }

        /// Returns the number of points on the map where at least two vents overlap.
        /// Only vents that are rows and columns are considered, unless `consider_diagonals` is `true`.
        pub fn overlapping(&self, consider_diagonals: bool) -> u32 {
            let mut map: HashMap<(usize, usize), u32> = HashMap::with_capacity(MAP_SIZE * MAP_SIZE);

            for vent in &self.0 {
                let (x_range, is_row): (Vec<usize>, bool) = match vent.p1.x.cmp(&vent.p2.x) {
                    Ordering::Equal => (iter::repeat(vent.p1.x).take(MAP_SIZE).collect(), true),
                    Ordering::Less => ((vent.p1.x..vent.p2.x + 1).collect(), false),
                    Ordering::Greater => ((vent.p2.x..vent.p1.x + 1).rev().collect(), false),
                };

                let (y_range, is_col): (Vec<usize>, bool) = match vent.p1.y.cmp(&vent.p2.y) {
                    Ordering::Equal => (iter::repeat(vent.p1.y).take(MAP_SIZE).collect(), true),
                    Ordering::Less => ((vent.p1.y..vent.p2.y + 1).collect(), false),
                    Ordering::Greater => ((vent.p2.y..vent.p1.y + 1).rev().collect(), false),
                };

                if is_row || is_col || consider_diagonals {
                    for point in x_range.into_iter().zip(y_range) {
                        map.entry(point)
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                }
            }

            map.values().fold(0, |mut overlapping, &n| {
                if n > 1 {
                    overlapping += 1;
                }
                overlapping
            })
        }
    }
}
