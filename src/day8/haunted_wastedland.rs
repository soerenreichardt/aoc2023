use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

enum Direction {
    Left,
    Right
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unknown direction '{}'", c)
        }
    }
}

pub fn steps_to_reach_z(input: &str) -> u32 {
    let (directions, network) = input.split_once("\n\n").unwrap();

    let directions = directions.chars().map(Direction::from).collect::<Vec<_>>();
    let network = network.lines()
        .map(|line| line.split_once(" = ").unwrap())
        .map(|(source, options)| (
            source,
            options
                .strip_prefix('(').unwrap()
                .strip_suffix(')').unwrap()
                .split_once(", ").unwrap()
        )).collect::<HashMap<&str, (&str, &str)>>();

    follow_network(directions, network)
}

const START: &str = "AAA";
const GOAL: &str = "ZZZ";
fn follow_network(directions: Vec<Direction>, network: HashMap<&str, (&str, &str)>) -> u32 {
    let mut steps_taken: u32 = 0;
    let mut next_elements = network.get(START).unwrap();
    'outer: loop {
        for direction in &directions {
            let chosen_element = match direction {
                Direction::Left => next_elements.0,
                Direction::Right => next_elements.1
            };
            steps_taken += 1;
            if chosen_element == GOAL {
                break 'outer;
            }
            next_elements = network.get(chosen_element).unwrap();
        }
    }

    steps_taken
}

#[cfg(test)]
mod tests {
    use crate::day8::haunted_wastedland::steps_to_reach_z;

    #[test]
    fn should_follow_network() {
        let input = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;
        assert_eq!(steps_to_reach_z(input), 2);
    }
}