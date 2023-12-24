mod lavaduct_lagoon;

pub fn part1() {
    let input = std::str::from_utf8(include_bytes!("../../resource/day18/input1")).unwrap();
    println!("{}", lavaduct_lagoon::cubic_meters_of_lava(input))
}