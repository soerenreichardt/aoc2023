use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

struct Universe {
    grid: Vec<Vec<Content>>,
}

#[derive(Clone, PartialEq)]
enum Content {
    Space,
    Galaxy,
    ExpandedSpace,
}

pub fn all_pairs_shortest_path(input: &str) -> usize {
    let universe = Universe::from_str(input).unwrap();
    let expanded_universe = universe.expand();
    let galaxy_positions = expanded_universe.galaxy_positions();
    Universe::galaxy_pairs(galaxy_positions)
        .into_iter()
        .map(|(source, target)| manhattan_distance(source, target))
        .sum()
}

pub fn all_pairs_million_times_expand_shortest_path(input: &str) -> usize {
    let universe = Universe::from_str(input).unwrap();
    let huge_universe = universe.huge_expand();
    let galaxy_positions = huge_universe.galaxy_positions();
    let (huge_rows, huge_columns) = huge_universe.huge_grid_elements();
    Universe::galaxy_pairs(galaxy_positions)
        .into_iter()
        .map(|(source, target)| modified_manhattan_distance(source, target, &huge_rows, &huge_columns))
        .sum()
}

impl Content {
    fn distance(&self) -> usize {
        match self {
            Content::Space | Content::Galaxy => 1,
            Content::ExpandedSpace => 1_000_000,
        }
    }
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
                    Content::Space=> write!(f, ".")?,
                    Content::ExpandedSpace => write!(f, "o")?,
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

    fn huge_expand(&self) -> Self {
        let mut grid = self.grid.clone();

        // scan for empty rows
        for (row, grid_row) in self.grid.iter().enumerate() {
            if grid_row.iter().all(|c| c == &Content::Space) {
                grid[row] = vec![Content::ExpandedSpace; self.grid[0].len()];
            }
        }

        // scan for empty columns
        let column_length = grid.len();
        for column in 0..self.grid[0].len() {
            let mut all_empty = true;
            for row in 0..column_length {
                match grid[row][column] {
                    Content::Galaxy => {
                        all_empty = false;
                        break;
                    }
                    Content::ExpandedSpace | Content::Space => continue
                }
            }

            if all_empty {
                for row in 0..column_length {
                    grid[row][column] = Content::ExpandedSpace;
                }
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

    fn huge_grid_elements(&self) -> (HashSet<usize>, HashSet<usize>) {
        let mut huge_rows = HashSet::new();
        for row in 0..self.grid.len() {
            if self.grid[row][0] == Content::ExpandedSpace {
                huge_rows.insert(row);
            }
        }

        let mut huge_columns = HashSet::new();
        for column in 0..self.grid[0].len() {
            if self.grid[0][column] == Content::ExpandedSpace {
                huge_columns.insert(column);
            }
        }

        (huge_rows, huge_columns)
    }

    fn galaxy_pairs(positions: Vec<(usize, usize)>) -> Vec<((usize, usize), (usize, usize))> {
        let mut pairs = Vec::new();
        for source in 0..positions.len() {
            for target in source + 1..positions.len() {
                pairs.push((positions[source], positions[target]))
            }
        }
        pairs
    }
}

fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    let (a_row, a_column) = a;
    let (b_row, b_column) = b;
    ((a_row as i32 - b_row as i32).abs() + (a_column as i32 - b_column as i32).abs()) as usize
}

fn modified_manhattan_distance(a: (usize, usize), b: (usize, usize), huge_rows: &HashSet<usize>, huge_columns: &HashSet<usize>) -> usize {
    let (a_row, a_column) = a;
    let (b_row, b_column) = b;
    let mut distance = ((a_row as i32 - b_row as i32).abs() + (a_column as i32 - b_column as i32).abs()) as usize;
    for row in a_row.min(b_row)..a_row.max(b_row) {
        if huge_rows.contains(&row) {
           distance += Content::ExpandedSpace.distance() - 1;
        }
    }
    for column in a_column.min(b_column)..a_column.max(b_column) {
        if huge_columns.contains(&column) {
           distance += Content::ExpandedSpace.distance() - 1;
        }
    }

    distance
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
        assert_eq!(super::all_pairs_shortest_path(input), 374);
    }

    #[test]
    fn should_huge_expand_image() {
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
        let expanded_universe = r#"..o#.o..o.
..o..o.#o.
#.o..o..o.
oooooooooo
..o..o#.o.
.#o..o..o.
..o..o..o#
oooooooooo
..o..o.#o.
#.o.#o..o.
"#;
        assert_eq!(universe.huge_expand().to_string(), expanded_universe);
    }

    #[test]
    fn should_sum_huge_galaxy_distances() {
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
        assert_eq!(super::all_pairs_million_times_expand_shortest_path(input), 8410);
    }
}