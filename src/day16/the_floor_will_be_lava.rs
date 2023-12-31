use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub fn energized_tiles(input: &str) -> usize {
    let mut board = Board::from(input);
    let beam = Beam { direction: Direction::Right, position: (0, 0) };
    board.traverse_beam_and_calculate_energized_tiles(beam)
}

pub fn maximum_energized_tiles(input: &str) -> usize {
    let mut board = Board::from(input);
    let height = board.tiles.len();
    let width = board.tiles[0].len();

    let mut max_energized_tiles = 0;

    for row in 0..height {
        let beam = Beam { direction: Direction::Right, position: (row, 0) };
        max_energized_tiles = max_energized_tiles.max(board.traverse_beam_and_calculate_energized_tiles(beam));

        let beam = Beam { direction: Direction::Left, position: (row, width - 1) };
        max_energized_tiles = max_energized_tiles.max(board.traverse_beam_and_calculate_energized_tiles(beam));
    }

    for col in 0..width {
        let beam = Beam { direction: Direction::Down, position: (0, col) };
        max_energized_tiles = max_energized_tiles.max(board.traverse_beam_and_calculate_energized_tiles(beam));

        let beam = Beam { direction: Direction::Up, position: (height - 1, col) };
        max_energized_tiles = max_energized_tiles.max(board.traverse_beam_and_calculate_energized_tiles(beam));
    }

    max_energized_tiles
}

#[derive(Clone, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Board {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Beam {
    direction: Direction,
    position: (usize, usize),
}

#[derive(Clone, Hash, PartialEq)]
enum Tile {
    Empty,
    MirrorUp,
    MirrorDown,
    SplitHorizontal,
    SplitVertical,
}

#[derive(Clone, PartialEq)]
enum EnergyState {
    Energized,
    Empty,
}

trait Reflect {
    fn reflect(&self, beam: &mut Beam) -> Vec<&mut Beam>;
}

impl Board {
    fn new(tiles: Vec<Vec<Tile>>) -> Self {
        Board { tiles }
    }

    fn print(&self, visited: &HashMap<(usize, usize), Vec<Direction>>) {
        for row in 0..self.tiles.len() {
            println!();
            for col in 0..self.tiles[0].len() {
                match &self.tiles[row][col] {
                    Tile::Empty => match visited.get(&(row, col)) {
                        Some(directions) => match directions.len() {
                            1 => print!("{}", directions[0]),
                            len if len > 1 => print!("{}", len),
                            _ => print!("."),
                        },
                        None => print!("."),
                    }
                    tile => print!("{}", tile),
                }
            }
        }
    }

    fn traverse_beam_and_calculate_energized_tiles(&self, beam: Beam) -> usize {
        let mut visited = HashMap::new();
        self.traverse_beam(beam, &mut visited);
        self.count_energized_tiles(&visited)
    }

    fn count_energized_tiles(&self, visited: &HashMap<(usize, usize), Vec<Direction>>) -> usize {
        let mut energized_tiles = 0;
        for row in 0..self.tiles.len() {
            for col in 0..self.tiles[0].len() {
                match visited.get(&(row, col)) {
                    Some(_) => energized_tiles += 1,
                    None => {}
                }
            }
        }
        energized_tiles
    }

    fn traverse_beam(&self, beam: Beam, visited: &mut HashMap<(usize, usize), Vec<Direction>>) {
        match self.calculate_reflected_beams(beam) {
            (b, None) => self.do_traverse_beam(b, visited),
            (b1, Some(b2)) => {
                self.do_traverse_beam(b1, visited);
                self.do_traverse_beam(b2, visited);
                return;
            }
        }
    }

    fn do_traverse_beam(&self, mut beam: Beam, visited: &mut HashMap<(usize, usize), Vec<Direction>>) {
        while !visited.entry(beam.position.clone()).or_insert(Vec::new()).contains(&beam.direction) {
            visited.get_mut(&beam.position).unwrap().push(beam.direction.clone());
            if !self.travel_beam(&mut beam) {
                return;
            }
            match self.calculate_reflected_beams(beam) {
                (b, None) => beam = b,
                (b1, Some(b2)) => {
                    self.do_traverse_beam(b1, visited);
                    self.do_traverse_beam(b2, visited);
                    return;
                }
            }
        }
    }

    fn calculate_reflected_beams(&self, mut beam: Beam) -> (Beam, Option<Beam>) {
        match self.tiles[beam.position.0][beam.position.1] {
            Tile::Empty => (beam, None),
            Tile::MirrorDown => {
                match beam.direction {
                    Direction::Up => beam.direction = Direction::Left,
                    Direction::Down => beam.direction = Direction::Right,
                    Direction::Left => beam.direction = Direction::Up,
                    Direction::Right => beam.direction = Direction::Down,
                }
                (beam, None)
            }
            Tile::MirrorUp => {
                match beam.direction {
                    Direction::Up => beam.direction = Direction::Right,
                    Direction::Down => beam.direction = Direction::Left,
                    Direction::Left => beam.direction = Direction::Down,
                    Direction::Right => beam.direction = Direction::Up,
                }
                (beam, None)
            }
            Tile::SplitHorizontal => {
                match beam.direction {
                    Direction::Left | Direction::Right => (beam, None),
                    Direction::Up | Direction::Down => {
                        let mut beam1 = beam.clone();
                        let mut beam2 = beam;

                        beam1.direction = Direction::Left;
                        beam2.direction = Direction::Right;

                        (beam1, Some(beam2))
                    }
                }
            }
            Tile::SplitVertical => {
                match beam.direction {
                    Direction::Up | Direction::Down => (beam, None),
                    Direction::Left | Direction::Right => {
                        let mut beam1 = beam.clone();
                        let mut beam2 = beam;

                        beam1.direction = Direction::Up;
                        beam2.direction = Direction::Down;

                        (beam1, Some(beam2))
                    }
                }
            }
        }
    }

    fn travel_beam(&self, beam: &mut Beam) -> bool {
        match beam.direction {
            Direction::Up if beam.position.0 == 0 => return false,
            Direction::Up => beam.position.0 -= 1,

            Direction::Down if beam.position.0 == self.tiles.len() - 1 => return false,
            Direction::Down => beam.position.0 += 1,

            Direction::Left if beam.position.1 == 0 => return false,
            Direction::Left => beam.position.1 -= 1,

            Direction::Right if beam.position.1 == self.tiles[0].len() - 1 => return false,
            Direction::Right => beam.position.1 += 1,
        }

        true
    }
}

impl From<&str> for Board {
    fn from(input: &str) -> Self {
        let tiles = input.lines().map(|line| line.chars().map(Tile::from).collect::<Vec<_>>()).collect::<Vec<_>>();
        Board::new(tiles)
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '/' => Tile::MirrorUp,
            '\\' => Tile::MirrorDown,
            '-' => Tile::SplitHorizontal,
            '|' => Tile::SplitVertical,
            _ => panic!("Invalid tile '{value}'")
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::MirrorUp => write!(f, "/"),
            Tile::MirrorDown => write!(f, "\\"),
            Tile::SplitHorizontal => write!(f, "-"),
            Tile::SplitVertical => write!(f, "|"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_count_energized_tiles() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

        assert_eq!(46, energized_tiles(input));
    }

    #[test]
    fn should_find_max_energized_tiles() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

        assert_eq!(51, maximum_energized_tiles(input));
    }


    #[test]
    fn should_visit_mirrors_twice() {
        let input = r#"..\..
/./..
\./.."#;

        assert_eq!(11, energized_tiles(input));
    }
}
