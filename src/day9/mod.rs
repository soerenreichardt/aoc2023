mod mirage_maintenance;

pub fn part1() {
    let input = std::str::from_utf8(include_bytes!("../../resource/day9/input1")).unwrap();
    println!("{}", mirage_maintenance::extrapolate(input));
}

pub fn part2() {
    let input = std::str::from_utf8(include_bytes!("../../resource/day9/input1")).unwrap();
    println!("{}", mirage_maintenance::extrapolate_backwards(input));
}