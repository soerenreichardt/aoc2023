use std::fmt::{Display, Formatter};
use std::str::FromStr;

struct Universe {
    grid: Vec<Vec<Content>>,
}

#[derive(Clone, PartialEq)]
enum Content {
    Space,
    Galaxy
}

pub fn all_pair_shortest_path(input: &str) -> usize {
    let universe = Universe::from_str(input).unwrap();
    let expanded_universe = universe.expand();
    let galaxy_positions = expanded_universe.galaxy_positions();
    Universe::all_pair_shortest_path(galaxy_positions).iter().sum()
}

impl FromStr for Universe {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s.lines().map(|line| {
            line.chars().map(|c| match c {
                '.' => Content::Space,
                '#' => Content::Galaxy,
                _ => panic!("Unknown content '{}'", c)
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();
        Ok(Universe { grid })
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for content in row {
                match content {
                    Content::Space => write!(f, ".")?,
                    Content::Galaxy => write!(f, "#")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Universe {
    fn expand(&self) -> Self {
        let mut grid = self.grid.clone();

        // scan for empty rows
        let mut row_offset = 0;
        let row_length = self.grid[0].len();
        for (row, grid_row) in self.grid.iter().enumerate() {
            if grid_row.iter().all(|c| c == &Content::Space) {
                grid.insert(row + row_offset, vec![Content::Space; row_length]);
                row_offset += 1;
            }
        }

        // scan for empty columns
        let mut column_offset = 0;
        let column_length = grid.len();
        for column in 0..self.grid[0].len() {
            let mut all_empty = true;
            for row in 0..column_length {
                if grid[row][column + column_offset] != Content::Space {
                    all_empty = false;
                    break;
                }
            }

            if all_empty {
                for row in 0..column_length {
                    grid[row].insert(column + column_offset, Content::Space);
                }
                column_offset += 1;
            }
        }

        Universe { grid }
    }

    fn galaxy_positions(&self) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for (row, grid_row) in self.grid.iter().enumerate() {
            for (column, content) in grid_row.iter().enumerate() {
                if content == &Content::Galaxy {
                    positions.push((row, column));
                }
            }
        }
        positions
    }

    fn all_pair_shortest_path(positions: Vec<(usize, usize)>) -> Vec<usize> {
        let mut distances = Vec::new();
        for source in 0..positions.len() {
            for target in source + 1..positions.len() {
                let distance = Self::manhattan_distance(positions[source], positions[target]);
                distances.push(distance)
            }
        }
        distances
    }

    fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
        let (a_row, a_column) = a;
        let (b_row, b_column) = b;
        ((a_row as i32 - b_row as i32).abs() + (a_column as i32 - b_column as i32).abs()) as usize
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::day11::cosmic_expansion::Universe;

    #[test]
    fn should_expand_image() {
        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
        let universe = Universe::from_str(input).unwrap();
        let expanded_universe = r#"....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......
"#;
        assert_eq!(universe.expand().to_string(), expanded_universe);
    }

    #[test]
    fn should_sum_galaxy_distances() {
        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
        assert_eq!(super::all_pair_shortest_path(input), 374);
    }
}