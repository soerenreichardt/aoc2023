mod seeds;

pub fn part1() {
    let input = std::str::from_utf8(include_bytes!("../../resource/day5/input1")).unwrap();
    println!("{}", seeds::sum_locations(input));
}