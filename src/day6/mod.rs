mod race;

pub fn part1() {
    let input = std::str::from_utf8(include_bytes!("../../resource/day6/input1")).unwrap();
    println!("{}", race::won_races(input));
}

pub fn part2() {
    let input = std::str::from_utf8(include_bytes!("../../resource/day6/input1")).unwrap();
    println!("{}", race::won_races_part2(input));
}
