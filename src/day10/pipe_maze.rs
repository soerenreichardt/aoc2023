use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

struct MainLoop {}

struct Maze {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Tile>>,
    start_position: (usize, usize),
}

#[derive(Debug, PartialEq)]
enum Tile {
    Air,
    Start,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

pub fn farthest_point_in_loop(input: &str) -> u32 {
    let maze = parse_maze(input);
    maze.farthest_distance_in_loop()
}

fn parse_maze(input: &str) -> Maze {
    let mut tiles = input
        .lines()
        .map(|line| line.chars().map(Tile::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = tiles.len();
    let width = tiles[0].len();

    let start_position = find_start_position(&tiles);
    let start_tile = interpolate_tile_at_start_position(&tiles, start_position);

    tiles[start_position.0][start_position.1] = start_tile;

    Maze {
        width,
        height,
        tiles,
        start_position
    }
}

fn find_start_position(tiles: &[Vec<Tile>]) -> (usize, usize) {
    tiles
        .iter().enumerate()
        .find_map(|(row, rows)| rows.iter().enumerate()
            .find_map(|(column, tile)| if tile == &Tile::Start {
                Some((row, column))
            } else {
                None
            })
        ).expect("Start position must be found")
}

const NORTH: usize = 1;
const SOUTH: usize = 2;
const EAST: usize = 4;
const WEST: usize = 8;

fn interpolate_tile_at_start_position(tiles: &[Vec<Tile>], (row, column): (usize, usize)) -> Tile {
    let mut connections = 0;
    if row > 0 {
        let top_neighbor = &tiles[row - 1][column];
        if [Tile::Vertical, Tile::SouthWest, Tile::SouthEast].contains(top_neighbor) {
            connections |= NORTH;
        }
    }
    if column > 0 {
        let left_neighbor = &tiles[row][column - 1];
        if [Tile::Horizontal, Tile::SouthEast, Tile::NorthEast].contains(left_neighbor) {
            connections |= WEST;
        }
    }
    if row < tiles.len() - 1 {
        let bottom_neighbor = &tiles[row + 1][column];
        if [Tile::Vertical, Tile::NorthEast, Tile::NorthWest].contains(bottom_neighbor) {
            connections |= SOUTH;
        }
    }
    if column < tiles[0].len() - 1 {
        let right_neighbor = &tiles[row][column + 1];
        if [Tile::Horizontal, Tile::SouthWest, Tile::NorthWest].contains(right_neighbor) {
            connections |= EAST;
        }
    }

    match connections {
        c if c == (NORTH | SOUTH) => Tile::Vertical,
        c if c == (EAST | WEST) => Tile::Horizontal,
        c if c == (NORTH | EAST) => Tile::NorthEast,
        c if c == (NORTH | WEST) => Tile::NorthWest,
        c if c == (SOUTH | EAST) => Tile::SouthEast,
        c if c == (SOUTH | WEST) => Tile::SouthWest,
        _ => panic!("Start position does not connect to exactly 2 neighbors. [{connections}]")
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Air,
            'S' => Tile::Start,
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            'F' => Tile::SouthEast,
            '7' => Tile::SouthWest,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            _ => panic!("Unrecognized tile {value}")
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Air => f.write_char('.'),
            Tile::Start => f.write_char('S'),
            Tile::Vertical => f.write_char('|'),
            Tile::Horizontal => f.write_char('-'),
            Tile::NorthEast => f.write_char('L'),
            Tile::NorthWest => f.write_char('J'),
            Tile::SouthEast => f.write_char('F'),
            Tile::SouthWest => f.write_char('7'),
        }
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.tiles.iter().enumerate() {
            for tile in row {
                f.write_str(format!("{tile}").as_str())?;
            }

            if i != self.tiles.len() - 1 {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

impl Maze {
    fn farthest_distance_in_loop(&self) -> u32 {
        let (start_row, start_col) = self.start_position;
        let start_tile = &self.tiles[start_row][start_col];
        let mut distances = HashMap::new();
        distances.insert((start_row, start_col), 0);

        let mut traverse_queue: Vec<((usize, usize), usize, u32)> = Vec::new();
        match start_tile {
            Tile::Vertical => {
                traverse_queue.push(((start_row - 1, start_col), SOUTH, 0));
                traverse_queue.push(((start_row + 1, start_col), NORTH, 0));
            }
            Tile::Horizontal => {
                traverse_queue.push(((start_row, start_col - 1), EAST, 0));
                traverse_queue.push(((start_row, start_col + 1), WEST, 0));
            }
            Tile::NorthEast => {
                traverse_queue.push(((start_row - 1, start_col), SOUTH, 0));
                traverse_queue.push(((start_row, start_col + 1), WEST, 0));
            }
            Tile::NorthWest => {
                traverse_queue.push(((start_row - 1, start_col), SOUTH, 0));
                traverse_queue.push(((start_row, start_col - 1), EAST, 0));
            }
            Tile::SouthEast => {
                traverse_queue.push(((start_row + 1, start_col), NORTH, 0));
                traverse_queue.push(((start_row, start_col + 1), WEST, 0));
            }
            Tile::SouthWest => {
                traverse_queue.push(((start_row + 1, start_col), NORTH, 0));
                traverse_queue.push(((start_row, start_col - 1), EAST, 0));
            }
            _ => panic!("Invalid start tile {start_tile}")
        }

        while let Some((next_position, from, distance_from_start)) = traverse_queue.pop() {
            self.traverse(next_position, from, &mut distances, distance_from_start, &mut traverse_queue);
        }

        let mut distance_values = distances.values().collect::<Vec<_>>();
        distance_values.sort();
        **distance_values.last().unwrap()
    }

    fn traverse(&self, (row, col): (usize, usize), from: usize, distances: &mut HashMap<(usize, usize), u32>, distance_from_start: u32, queue: &mut Vec<((usize, usize), usize, u32)>) {
        let distance_from_start = distance_from_start + 1;
        match distances.get_mut(&(row, col)) {
            Some(distance) if distance_from_start < *distance => *distance = distance_from_start,
            Some(_) => return,
            None => { distances.insert((row, col), distance_from_start); }
        };
        let (next_position, from) = match (&self.tiles[row][col], from) {
            (Tile::Horizontal, EAST) => ((row, col - 1), EAST),
            (Tile::Horizontal, WEST) => ((row, col + 1), WEST),
            (Tile::Vertical, SOUTH) => ((row - 1, col), SOUTH),
            (Tile::Vertical, NORTH) => ((row + 1, col), NORTH),
            (Tile::NorthEast, NORTH) => ((row, col + 1), WEST),
            (Tile::NorthEast, EAST) => ((row - 1, col), SOUTH),
            (Tile::NorthWest, NORTH) => ((row, col - 1), EAST),
            (Tile::NorthWest, WEST) => ((row - 1, col), SOUTH),
            (Tile::SouthEast, SOUTH) => ((row, col + 1), WEST),
            (Tile::SouthEast, EAST) => ((row + 1, col), NORTH),
            (Tile::SouthWest, SOUTH) => ((row, col - 1), EAST),
            (Tile::SouthWest, WEST) => ((row + 1, col), NORTH),
            (tile, from) => panic!("({row}, {col}) {tile} - {from}")
        };

        queue.push((next_position, from, distance_from_start));
    }
}

#[cfg(test)]
mod tests {
    use crate::day10::pipe_maze::{parse_maze, Tile};

    #[test]
    fn should_parse_maze() {
        let input = r#".....
.S-7.
.|.|.
.L-J.
....."#;
        let maze = parse_maze(input);
        let expected = r#".....
.F-7.
.|.|.
.L-J.
....."#;
        assert_eq!(maze.to_string(), expected)
    }

    #[test]
    fn should_traverse_maze() {
        let input = r#".....
.S-7.
.|.|.
.L-J.
....."#;
        let maze = parse_maze(input);
        assert_eq!(maze.farthest_distance_in_loop(), 4);
    }

    #[test]
    fn should_traverse_complex_maze() {
        let input = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;
        let maze = parse_maze(input);
        assert_eq!(maze.farthest_distance_in_loop(), 8);
    }

    #[test]
    fn should_interpolate_start_tile() {
        let input = r#"L7|
7S|
J||"#;
        let maze = parse_maze(input);
        let (start_row, start_col) = maze.start_position;
        assert_eq!(maze.tiles[start_row][start_col], Tile::Vertical)
    }
}

