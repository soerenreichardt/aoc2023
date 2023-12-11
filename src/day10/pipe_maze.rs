use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

struct MainLoop {}

struct Maze {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Tile>>,
    start_position: (usize, usize),
}

#[derive(Debug, PartialEq, Eq)]
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

pub fn enclosed_tiles(input: &str) -> u32 {
    let maze = parse_maze(input);
    let main_loop = maze.calculate_main_loop();
    maze.enclosed_tiles(main_loop).len() as u32
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
            self.traverse(next_position, from, distance_from_start, &mut traverse_queue, |(row, col), dst, _| match distances.get_mut(&(row, col)) {
                Some(distance) if distance_from_start < *distance => {
                    *distance = distance_from_start;
                    true
                },
                Some(_) => false,
                None => {
                    distances.insert((row, col), distance_from_start);
                    true
                }
            });
        }

        let mut distance_values = distances.values().collect::<Vec<_>>();
        distance_values.sort();
        **distance_values.last().unwrap()
    }

    fn traverse<F: FnMut((usize, usize), u32, usize) -> bool>(&self, (row, col): (usize, usize), from: usize, distance_from_start: u32, queue: &mut Vec<((usize, usize), usize, u32)>, mut traverse_fn: F) {
        let distance_from_start = distance_from_start + 1;
        if !traverse_fn((row, col), distance_from_start, from) {
            return
        }

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

    fn calculate_main_loop(&self) -> HashSet<(usize, usize)> {
        let (start_row, start_col) = self.start_position;
        let start_tile = &self.tiles[start_row][start_col];
        let mut traverse_queue = Vec::new();

        match start_tile {
            Tile::Vertical => traverse_queue.push(((start_row - 1, start_col), SOUTH, 0)),
            Tile::Horizontal => traverse_queue.push(((start_row, start_col - 1), EAST, 0)),
            Tile::NorthEast => traverse_queue.push(((start_row - 1, start_col), SOUTH, 0)),
            Tile::NorthWest => traverse_queue.push(((start_row - 1, start_col), SOUTH, 0)),
            Tile::SouthEast => traverse_queue.push(((start_row + 1, start_col), NORTH, 0)),
            Tile::SouthWest => traverse_queue.push(((start_row + 1, start_col), NORTH, 0)),
            _ => panic!("Invalid start tile {start_tile}")
        };

        let mut main_loop = HashSet::new();

        while let Some((next_position, from, distance_from_start)) = traverse_queue.pop() {
            self.traverse(next_position, from, distance_from_start, &mut traverse_queue, |pos, _, _| main_loop.insert(pos));
        }

        main_loop
    }

    fn enclosed_tiles(&self, main_loop: HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
        let (start_row, start_col) = self.start_position;
        let start_tile = &self.tiles[start_row][start_col];
        let mut traverse_queue = Vec::new();

        match start_tile {
            Tile::Vertical => traverse_queue.push(((start_row - 1, start_col), SOUTH, 0)),
            Tile::Horizontal => traverse_queue.push(((start_row, start_col - 1), EAST, 0)),
            Tile::NorthEast => traverse_queue.push(((start_row - 1, start_col), SOUTH, 0)),
            Tile::NorthWest => traverse_queue.push(((start_row - 1, start_col), SOUTH, 0)),
            Tile::SouthEast => traverse_queue.push(((start_row + 1, start_col), NORTH, 0)),
            Tile::SouthWest => traverse_queue.push(((start_row + 1, start_col), NORTH, 0)),
            _ => panic!("Invalid start tile {start_tile}")
        };

        let mut enclosed = HashSet::new();
        while let Some((next_position, from, distance_from_start)) = traverse_queue.pop() {
            self.traverse(next_position, from, distance_from_start, &mut traverse_queue, |pos, _, from| {
                if pos == self.start_position {
                    return false;
                }
                if (from == SOUTH && [Tile::Vertical, Tile::SouthWest].contains(&self.tiles[pos.0][pos.1]))
                    || (from == WEST && &self.tiles[pos.0][pos.1] == &Tile::NorthWest) {
                    let mut col = pos.1 + 1;
                    loop {
                        if main_loop.contains(&(pos.0, col)) {
                            return true;
                        }
                        if col == self.width {
                            enclosed.clear();
                            return false;
                        }

                        enclosed.insert((pos.0, col));
                        col += 1;
                    }
                }
                true
            });
        }

        if enclosed.len() != 0 {
            return enclosed;
        }

        let mut traverse_queue = Vec::new();
        match start_tile {
            Tile::Vertical => traverse_queue.push(((start_row - 1, start_col), SOUTH, 0)),
            Tile::Horizontal => traverse_queue.push(((start_row, start_col - 1), EAST, 0)),
            Tile::NorthEast => traverse_queue.push(((start_row - 1, start_col), SOUTH, 0)),
            Tile::NorthWest => traverse_queue.push(((start_row - 1, start_col), SOUTH, 0)),
            Tile::SouthEast => traverse_queue.push(((start_row + 1, start_col), NORTH, 0)),
            Tile::SouthWest => traverse_queue.push(((start_row + 1, start_col), NORTH, 0)),
            _ => panic!("Invalid start tile {start_tile}")
        };

        while let Some((next_position, from, distance_from_start)) = traverse_queue.pop() {
            self.traverse(next_position, from, distance_from_start, &mut traverse_queue, |pos, _, from| {
                if pos == self.start_position {
                    return false;
                }
                if (from == SOUTH && [Tile::Vertical, Tile::SouthEast].contains(&self.tiles[pos.0][pos.1]))
                    || (from == EAST && &self.tiles[pos.0][pos.1] == &Tile::NorthEast) {
                    let mut col = pos.1 - 1;
                    loop {
                        if main_loop.contains(&(pos.0, col)) {
                            return true;
                        }
                        if col == 0 {
                            enclosed.clear();
                            return false;
                        }

                        enclosed.insert((pos.0, col));
                        col -= 1;
                    }
                }
                true
            });
        }

        enclosed
    }
}

impl Tile {
    fn connects_horizontally_to_previous(&self, other: &Tile) -> bool {
        match self {
            Tile::Horizontal | Tile::NorthWest | Tile::SouthWest if [Tile::NorthEast, Tile::SouthEast, Tile::Horizontal].contains(&other) => true,
            _ => false
        }
    }

    fn connects_vertically_to_previous(&self, other: &Tile) -> bool {
        match self {
            Tile::Vertical | Tile::NorthEast | Tile::NorthWest if [Tile::Vertical, Tile::SouthWest, Tile::SouthEast].contains(&other) => true,
            _ => false
        }
    }
}

#[derive(Debug)]
enum SweepState {
    Open,
    Closed
}

impl SweepState {
    fn toggle(&mut self) -> SweepState {
        match self {
            SweepState::Open => SweepState::Closed,
            SweepState::Closed => SweepState::Open
        }
    }

    fn is_open(&self) -> bool {
        match self {
            SweepState::Open => true,
            SweepState::Closed => false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day10::pipe_maze::{enclosed_tiles, parse_maze, Tile};

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

    #[test]
    fn should_count_enclosed_tiles() {
        let input = r#"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."#;

        assert_eq!(enclosed_tiles(input), 4);
    }

    #[test]
    fn should_count_complex_enclosed_tiles() {
        let input = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#;

        assert_eq!(enclosed_tiles(input), 8);
    }

    #[test]
    fn should_count_complex_enclosed_tiles_2() {
        let input = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;

        assert_eq!(enclosed_tiles(input), 10);
    }
}

