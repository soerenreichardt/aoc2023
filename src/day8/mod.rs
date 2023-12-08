mod haunted_wastedland;

pub fn part1() {
    let input = std::str::from_utf8(include_bytes!("../../resource/day8/input1")).unwrap();
    println!("{}", haunted_wastedland::steps_to_reach_z(input));
}

pub fn part2() {
    let input = std::str::from_utf8(include_bytes!("../../resource/day8/input1")).unwrap();
    println!("{}", haunted_wastedland::parallel_steps_to_reach_z(input));
}