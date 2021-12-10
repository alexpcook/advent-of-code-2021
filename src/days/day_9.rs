use heightmap::HeightMap;
use std::path::Path;

pub fn main() {
    println!("### day 9 ###");

    let input_path = Path::new("./day_9.txt");
    let height_map = HeightMap::from_file(input_path).expect("could not read input file");

    // Part 1
    println!("part 1: risk level = {}", height_map.risk_level());
}

mod heightmap {
    use std::collections::HashMap;
    use std::fs;
    use std::io;
    use std::path::Path;

    /// Width/height of the lava tube map.
    const MAP_SIZE: usize = 100;

    /// A lava tube height map.
    #[derive(Debug)]
    pub struct HeightMap(HashMap<(usize, usize), u32>);

    impl HeightMap {
        /// Constructs a lava tube map from a file.
        pub fn from_file(path: &Path) -> io::Result<HeightMap> {
            let mut height_map = HashMap::with_capacity(MAP_SIZE * MAP_SIZE);

            let contents = fs::read_to_string(path)?;
            for (x, line) in contents.lines().enumerate() {
                for (y, c) in line.chars().enumerate() {
                    height_map.insert(
                        (x, y),
                        c.to_digit(10).ok_or_else(|| {
                            io::Error::new(
                                io::ErrorKind::Other,
                                format!("could not parse digit: {}", c),
                            )
                        })?,
                    );
                }
            }

            Ok(HeightMap(height_map))
        }

        /// Calculates the total risk level of the lava tube height map.
        pub fn risk_level(&self) -> u32 {
            let mut risk_level = 0;

            for i in 0..MAP_SIZE {
                for j in 0..MAP_SIZE {
                    let height = match self.0.get(&(i, j)) {
                        Some(h) => h,
                        None => continue,
                    };
                    if self.adjacent_heights(&(i, j)).iter().all(|h| height < h) {
                        risk_level += 1 + height;
                    }
                }
            }

            risk_level
        }

        /// Gets a point's adjacent heights given a `position` in the map.
        fn adjacent_heights(&self, position: &(usize, usize)) -> Vec<u32> {
            let (x, y) = *position;
            let mut adjacent_coordinates = vec![];

            if let Some(x) = x.checked_sub(1) {
                adjacent_coordinates.push((x, y));
            }
            if let Some(x) = x.checked_add(1) {
                adjacent_coordinates.push((x, y));
            }
            if let Some(y) = y.checked_sub(1) {
                adjacent_coordinates.push((x, y));
            }
            if let Some(y) = y.checked_add(1) {
                adjacent_coordinates.push((x, y));
            }

            let mut adjacent_heights = Vec::with_capacity(4);
            for coordinate in adjacent_coordinates {
                if let Some(result) = self.0.get(&coordinate) {
                    adjacent_heights.push(*result);
                }
            }

            adjacent_heights
        }
    }
}
