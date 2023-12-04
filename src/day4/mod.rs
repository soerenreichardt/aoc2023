mod scratch_cards;

pub fn part1() {
    let input = std::str::from_utf8(include_bytes!("../../resource/day4/input1")).unwrap();
    println!("{}", scratch_cards::winning_points(input));
}