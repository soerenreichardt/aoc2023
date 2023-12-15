mod parabolic_reflector_dish;

pub fn part1() {
    let input = std::str::from_utf8(include_bytes!("../../resource/day14/input1")).unwrap();
    println!("{}", parabolic_reflector_dish::calculate_load(input))
}