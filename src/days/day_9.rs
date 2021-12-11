use heightmap::HeightMap;
use std::path::Path;

pub fn main() {
    println!("### day 9 ###");

    let input_path = Path::new("./day_9.txt");
    let height_map = HeightMap::from_file(input_path).expect("could not read input file");

    // Part 1
    println!("part 1: risk level = {}", height_map.risk_level());

    // Part 2
    println!(
        "part 2: three largest basins product = {}",
        height_map.three_largest_basins_product()
    );
}

mod heightmap {
    use std::collections::{HashMap, HashSet};
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

        /// Calculates the product of the size of the three largest basins.
        pub fn three_largest_basins_product(&self) -> u32 {
            let mut basins = vec![];
            let mut visited_positions = HashSet::with_capacity(MAP_SIZE * MAP_SIZE);

            for i in 0..MAP_SIZE {
                for j in 0..MAP_SIZE {
                    if matches!(self.0.get(&(i, j)), Some(&h) if h < 9) {
                        basins.push(self.basin_size(&(i, j), &mut visited_positions));
                    }
                }
            }

            basins.sort_unstable();
            basins.into_iter().rev().take(3).product()
        }

        /// Calculates the size of the basin containing `position`. Uses `visited_positions`
        /// to keep track of which coordinates have already been visited on the map.
        fn basin_size(
            &self,
            position: &(usize, usize),
            visited_positions: &mut HashSet<(usize, usize)>,
        ) -> u32 {
            let mut size = 1;
            visited_positions.insert(*position);

            for point in self.adjacent_points(position) {
                if matches!(self.0.get(&point), Some(&x) if x < 9 && visited_positions.get(&point).is_none())
                {
                    visited_positions.insert(point);
                    size += self.basin_size(&point, visited_positions);
                }
            }

            size
        }

        /// Gets a point's adjacent heights given a `position` on the map.
        fn adjacent_heights(&self, position: &(usize, usize)) -> Vec<u32> {
            let mut adjacent_heights = Vec::with_capacity(4);

            for coordinate in self.adjacent_points(position) {
                if let Some(&result) = self.0.get(&coordinate) {
                    adjacent_heights.push(result);
                }
            }

            adjacent_heights
        }

        /// Gets a point's adjacent points given a `position` in the map.
        fn adjacent_points(&self, position: &(usize, usize)) -> Vec<(usize, usize)> {
            let &(x, y) = position;
            let mut adjacent_points = vec![];

            if let Some(x) = x.checked_sub(1) {
                adjacent_points.push((x, y));
            }
            if let Some(y) = y.checked_sub(1) {
                adjacent_points.push((x, y));
            }
            match x.checked_add(1) {
                Some(x) if x < MAP_SIZE => adjacent_points.push((x, y)),
                _ => {}
            }
            match y.checked_add(1) {
                Some(y) if y < MAP_SIZE => adjacent_points.push((x, y)),
                _ => {}
            }

            adjacent_points
        }
    }
}
