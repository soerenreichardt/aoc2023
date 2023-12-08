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
    let (directions, network) = parse_input(input);
    follow_network(directions, network)
}

pub fn parallel_steps_to_reach_z(input: &str) -> usize {
    let (directions, network) = parse_input(input);
    parallel_follow(directions, network)
}

fn parse_input(input: &str) -> (Vec<Direction>, HashMap<&str, (&str, &str)>) {
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
    (directions, network)
}

const START: &str = "AAA";
const GOAL: &str = "ZZZ";
fn follow_network(directions: Vec<Direction>, network: HashMap<&str, (&str, &str)>) -> u32 {
    solve_single_start(&directions, &network, START, |s| s == GOAL).1
}

fn parallel_follow(directions: Vec<Direction>, network: HashMap<&str, (&str, &str)>) -> usize {
    let mut start_positions = network.keys().filter(|node| node.ends_with('A')).map(|&node| node).collect::<Vec<_>>();

    let mut steps: Vec<usize> = start_positions.iter().map(|start_position| {
        // paths to goals repeat after the second iteration, so no need to traverse any farther
        let (goal_element, steps_taken, dir_index) = solve_single_start(&directions, &network, start_position, |s| s.ends_with('Z'));
        println!("{} -> {} {}", start_position, goal_element, steps_taken);
        steps_taken as usize
    }).collect();

    // use a calculator from here :/
    todo!()
}

fn solve_single_start<'a>(
    directions: &[Direction],
    network: &HashMap<&'a str, (&'a str, &'a str)>,
    start: &str,
    predicate: fn(&str) -> bool
) -> (&'a str, u32, usize) {
    let mut steps_taken: u32 = 0;
    let mut next_elements = network.get(start).unwrap();
    loop {
        for (i, direction) in directions.iter().enumerate() {
            let chosen_element = match direction {
                Direction::Left => next_elements.0,
                Direction::Right => next_elements.1
            };
            steps_taken += 1;
            if predicate(chosen_element) {
                return (chosen_element, steps_taken, i+1);
            }
            next_elements = network.get(chosen_element).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day8::haunted_wastedland::{parallel_steps_to_reach_z, steps_to_reach_z};

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

    #[test]
    fn should_follow_network_parallel() {
        let input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;
        assert_eq!(parallel_steps_to_reach_z(input), 6);
    }
}