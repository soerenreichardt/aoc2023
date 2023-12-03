mod gears;

pub fn part1() {
    let input = std::str::from_utf8(include_bytes!("../../resource/day3/input1")).unwrap();
    println!("{}", gears::gear_ratio(input));
}