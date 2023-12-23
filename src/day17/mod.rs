mod clumsy_crucible;

pub fn part1() {
    let input = std::str::from_utf8(include_bytes!("../../resource/day17/input1")).unwrap();
    println!("{}", clumsy_crucible::least_heat_loss(input))
}