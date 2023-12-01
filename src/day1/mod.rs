mod trebuchet;

pub fn part1() {
    let input = std::str::from_utf8(include_bytes!("../../resource/day1/input")).unwrap();
    println!("{}", trebuchet::process_input(input));
}