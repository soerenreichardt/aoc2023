use std::clone;

mod cubes;

pub fn part2() {
    let input = std::str::from_utf8(include_bytes!("../../resource/day2/input1")).unwrap();
    println!("{}", cubes::fewest_cubes(input));
}