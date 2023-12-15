use std::fmt::{Debug, Display, Formatter};

pub fn calculate_load(input: &str) -> usize {
    let mut dish = parse_input(input);
    tilt_upwards(&mut dish);
    sum_load(&dish)
}

fn parse_input(input: &str) -> Vec<Vec<PlatformState>> {
    input
        .lines()
        .map(|line| line.chars().map(PlatformState::from).collect::<Vec<_>>()).collect::<Vec<_>>()
}

fn tilt_upwards(dish: &mut Vec<Vec<PlatformState>>) {
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
    fn should_tilt_upwards() {
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
        tilt_upwards(&mut dish);
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
}