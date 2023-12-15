mod point_of_incidence;

pub fn part1() {
    let input = std::str::from_utf8(include_bytes!("../../resource/day13/input1")).unwrap();
    println!("{}", point_of_incidence::reflecting_notes_count(input))
}