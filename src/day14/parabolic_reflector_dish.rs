use std::fmt::{Debug, Display, Formatter};

pub fn calculate_load(input: &str) -> usize {
    let mut dish = parse_input(input);
    tilt_up(&mut dish);
    sum_load(&dish)
}

pub fn calculate_load_after_cycles(input: &str, cycles: usize) -> usize {
    let mut dish = parse_input(input);
    tilt_platform(&mut dish, cycles);
    sum_load(&dish)
}

fn parse_input(input: &str) -> Vec<Vec<PlatformState>> {
    input
        .lines()
        .map(|line| line.chars().map(PlatformState::from).collect::<Vec<_>>()).collect::<Vec<_>>()
}

fn tilt_platform(dish: &mut Vec<Vec<PlatformState>>, rotations: usize) {
    (0..rotations).for_each(|i| {
        tilt_up(dish);
        tilt_left(dish);
        tilt_down(dish);
        tilt_right(dish);
    });
}

fn tilt_up(dish: &mut Vec<Vec<PlatformState>>) {
    let mut row_state = vec![0; dish[0].len()];

    for row in 0..dish.len() {
        for col in 0..dish[0].len() {
            match dish[row][col] {
                PlatformState::RoundedRock => {
                    dish[row_state[col]][col] = PlatformState::RoundedRock;
                    if row_state[col] != row {
                        dish[row][col] = PlatformState::Empty;
                    }
                    row_state[col] += 1;
                },
                PlatformState::CubeRock => row_state[col] = row + 1,
                PlatformState::Empty => {}
            }
        }
    }
}

fn tilt_down(dish: &mut Vec<Vec<PlatformState>>) {
    let mut row_state = vec![dish.len() - 1; dish[0].len()];

    for row in (0..dish.len()).rev() {
        for col in 0..dish[0].len() {
            match dish[row][col] {
                PlatformState::RoundedRock => {
                    dish[row_state[col]][col] = PlatformState::RoundedRock;
                    if row_state[col] != row {
                        dish[row][col] = PlatformState::Empty;
                    }
                    row_state[col] = (row_state[col] as i64 - 1).max(0) as usize;
                },
                PlatformState::CubeRock => row_state[col] = (row as i64 - 1).max(0) as usize,
                PlatformState::Empty => {}
            }
        }
    }
}

fn tilt_left(dish: &mut Vec<Vec<PlatformState>>) {
    let mut col_state = vec![0; dish.len()];

    for row in 0..dish.len() {
        for col in 0..dish[0].len() {
            match dish[row][col] {
                PlatformState::RoundedRock => {
                    dish[row][col_state[row]] = PlatformState::RoundedRock;
                    if col_state[row] != col {
                        dish[row][col] = PlatformState::Empty;
                    }
                    col_state[row] += 1;
                },
                PlatformState::CubeRock => col_state[row] = col + 1,
                PlatformState::Empty => {}
            }
        }
    }
}

fn tilt_right(dish: &mut Vec<Vec<PlatformState>>) {
    let mut col_state = vec![dish[0].len() - 1; dish.len()];

    for row in 0..dish.len() {
        for col in (0..dish[0].len()).rev() {
            match dish[row][col] {
                PlatformState::RoundedRock => {
                    dish[row][col_state[row]] = PlatformState::RoundedRock;
                    if col_state[row] != col {
                        dish[row][col] = PlatformState::Empty;
                    }
                    col_state[row] = (col_state[row] as i64 - 1).max(0) as usize;
                },
                PlatformState::CubeRock => col_state[row] = (col as i64 - 1).max(0) as usize,
                PlatformState::Empty => {}
            }
        }
    }
}

fn sum_load(dish: &Vec<Vec<PlatformState>>) -> usize {
    let height = dish.len();
    dish.iter().enumerate().map(|(row_num, row)| row.iter().filter_map(|state| match state {
        PlatformState::RoundedRock => Some(height - row_num),
        _ => None
    }).sum::<usize>()).sum()
}

#[derive(PartialEq)]
enum PlatformState {
    RoundedRock,
    CubeRock,
    Empty
}

impl From<char> for PlatformState {
    fn from(value: char) -> Self {
        match value {
            '.' => PlatformState::Empty,
            '#' => PlatformState::CubeRock,
            'O' => PlatformState::RoundedRock,
            _ => panic!("Invalid platform state")
        }
    }
}

impl Debug for PlatformState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PlatformState::RoundedRock => write!(f, "O"),
            PlatformState::CubeRock => write!(f, "#"),
            PlatformState::Empty => write!(f, "."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_tilt_up() {
        let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

        let tilted = r#"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."#;
        let mut dish = parse_input(input);
        tilt_up(&mut dish);
        assert_eq!(parse_input(tilted), dish);
    }

    #[test]
    fn should_tilt_down() {
        let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

        let tilted = r#".....#....
....#....#
...O.##...
...#......
O.O....O#O
O.#..O.#.#
O....#....
OO....OO..
#OO..###..
#OO.O#...O"#;
        let mut dish = parse_input(input);
        tilt_down(&mut dish);
        assert_eq!(parse_input(tilted), dish);
    }

    #[test]
    fn should_tilt_left() {
        let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

        let tilted = r#"O....#....
OOO.#....#
.....##...
OO.#OO....
OO......#.
O.#O...#.#
O....#OO..
O.........
#....###..
#OO..#...."#;
        let mut dish = parse_input(input);
        tilt_left(&mut dish);
        assert_eq!(parse_input(tilted), dish);
    }

    #[test]
    fn should_tilt_right() {
        let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

        let tilted = r#"....O#....
.OOO#....#
.....##...
.OO#....OO
......OO#.
.O#...O#.#
....O#..OO
.........O
#....###..
#..OO#...."#;
        let mut dish = parse_input(input);
        tilt_right(&mut dish);
        assert_eq!(parse_input(tilted), dish);
    }

    #[test]
    fn should_calculate_load() {
        let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;
        assert_eq!(calculate_load(input), 136);
    }

    #[test]
    fn should_calculate_load_after_cycles() {
        let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;
        assert_eq!(calculate_load_after_cycles(input, 1000), 64);
    }

    #[test]
    fn should_tilt_1_cycle() {
        let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;
        let expected = r#".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."#;
        let mut dish = parse_input(input);
        tilt_platform(&mut dish, 1);
        assert_eq!(parse_input(expected), dish)
    }
}