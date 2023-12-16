mod lens_library;

pub fn part1() {
    let input = std::str::from_utf8(include_bytes!("../../resource/day15/input1")).unwrap();
    println!("{}", lens_library::sum_hash_values(input))
}