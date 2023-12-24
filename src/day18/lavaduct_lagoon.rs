use std::fmt::{Debug, Formatter};
use std::str::FromStr;

struct Map<'a> {
    tiles: Vec<Vec<Tile<'a>>>,
    bounds: Bounds,
}

#[derive(Clone, Debug, PartialEq)]
enum Tile<'a> {
    Ground,
    Trench(&'a str),
}

#[derive(Debug)]
struct DigPlan<'a> {
    instructions: Vec<Instruction<'a>>,
}

#[derive(Debug)]
struct Instruction<'a> {
    direction: Direction,
    amount: usize,
    color: &'a str,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Bounds {
    min_row: i64,
    max_row: usize,
    min_col: i64,
    max_col: usize,
}

pub fn cubic_meters_of_lava(input: &str) -> usize {
    let dig_plan = DigPlan::from(input);
    let mut map = Map::create_from_bounds(dig_plan.calculate_bounds());
    dig_plan.instructions.into_iter().fold((0, 0), |position, instruction| {
        map.execute_instruction(instruction, position)
    });
    // println!("{:?}", map);
    map.count_enclosed_tiles()
}

impl<'a> Map<'a> {
    fn create_from_bounds(bounds: Bounds) -> Self {
        let tiles = vec![vec![Tile::Ground; bounds.width()]; bounds.height()];
        Map { tiles, bounds }
    }

    fn execute_instruction(&mut self, instruction: Instruction<'a>, (start_row, start_col): (usize, usize)) -> (usize, usize) {
        let (row_increment, col_increment): (i64, i64) = match instruction.direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };
        for i in 0..instruction.amount as i64 {
            let row = start_row as i64 + row_increment * i;
            let col = start_col as i64 + col_increment * i;
            self.tiles[(self.bounds.row_offset() + row) as usize]
                [(self.bounds.col_offset() + col) as usize] = Tile::Trench(instruction.color);
        }

        ((start_row as i64 + row_increment * instruction.amount as i64) as usize,
         (start_col as i64 + col_increment * instruction.amount as i64) as usize)
    }

    fn count_enclosed_tiles(&self) -> usize {
        let mut counter = 0;
        let mut row_above = vec![0; self.bounds.width()];
        for (row_num, row) in self.tiles.iter().enumerate() {
            let mut intersections = 0;
            let mut last_tile_was_trench = false;
            for (col_num, tile) in row.iter().enumerate() {
                match tile {
                    Tile::Ground => {
                        if intersections % 2 == 1 {
                            counter += 1;
                        }
                        last_tile_was_trench = false;
                    }
                    Tile::Trench(_) => {
                        if !last_tile_was_trench {
                            intersections += 1;
                            last_tile_was_trench = true;
                        } else {
                            // if we are at the end of some consecutive trenches, check if
                            // the above tile was enclosed, if no, intersections should be even
                            // so that the tiles to the right are not enclosed,
                            // else, if the current tiles is not enclosed, it should be enclosed
                            if col_num < self.bounds.width() - 1 && self.tiles[row_num][col_num + 1] == Tile::Ground {
                                if !Self::is_enclosed(row_above[col_num]) {
                                    intersections = 0;
                                } else if !Self::is_enclosed(intersections) {
                                    intersections += 1;
                                }
                            }
                        }
                        counter += 1;
                    }
                }
                row_above[col_num] = intersections;
            }
        }

        counter
    }

    fn is_enclosed(intersections: i64) -> bool {
        intersections % 2 == 1
    }
}

impl<'a> DigPlan<'a> {
    fn calculate_bounds(&self) -> Bounds {
        let mut min_row = 0;
        let mut max_row = 0;
        let mut min_col = 0;
        let mut max_col = 0;

        let mut row: i64 = 0;
        let mut col: i64 = 0;

        for instruction in &self.instructions {
            let amount = instruction.amount as i64;
            match instruction.direction {
                Direction::Up => row -= amount,
                Direction::Down => row += amount,
                Direction::Left => col -= amount,
                Direction::Right => col += amount,
            }

            min_row = min_row.min(row);
            max_row = max_row.max(row);
            min_col = min_col.min(col);
            max_col = max_col.max(col);
        }

        Bounds::new(min_row, (max_row + 1) as usize, min_col, (max_col + 1) as usize)
    }
}

impl Bounds {
    fn new(min_row: i64, max_row: usize, min_col: i64, max_col: usize) -> Self {
        assert!(min_row <= 0);
        assert!(min_col <= 0);
        assert!(max_row as i64 >= min_row);
        assert!(max_col as i64 >= min_col);
        Bounds { min_row, max_row, min_col, max_col }
    }

    fn width(&self) -> usize {
        self.max_col + self.col_offset() as usize
    }

    fn height(&self) -> usize {
        self.max_row + self.row_offset() as usize
    }

    fn row_offset(&self) -> i64 {
        i64::abs(self.min_row)
    }

    fn col_offset(&self) -> i64 {
        i64::abs(self.min_col)
    }
}

impl<'a> From<&'a str> for DigPlan<'a> {
    fn from(s: &'a str) -> Self {
        let instructions = s
            .lines()
            .map(|line| Instruction::from(line))
            .collect::<Vec<_>>();
        DigPlan { instructions }
    }
}

impl<'a> From<&'a str> for Instruction<'a> {
    fn from(s: &'a str) -> Self {
        let row: [&str; 3] = s.split_ascii_whitespace().collect::<Vec<_>>().try_into().unwrap();
        let direction = match row[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction")
        };
        let amount = row[1].parse::<usize>().unwrap();
        let color = &row[2][1..row[2].len() - 1];
        Instruction { direction, amount, color }
    }
}

impl<'a> Debug for Map<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                match tile {
                    Tile::Ground => write!(f, ".")?,
                    Tile::Trench(_) => write!(f, "#")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cubic_meters_of_lava() {
        let input = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

        assert_eq!(cubic_meters_of_lava(input), 62);
    }

    #[test]
    fn test_cubic_meters_of_lava_2() {
        let input = r#"L 6 (#70c710)
U 5 (#0dc571)
R 6 (#5713f0)
D 5 (#d2c081)"#;

        assert_eq!(cubic_meters_of_lava(input), 6 * 7);
    }
}
