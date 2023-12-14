mod hot_springs;

pub fn part1() {
    let input = std::str::from_utf8(include_bytes!("../../resource/day12/input1")).unwrap();
    println!("{}", hot_springs::count_arrangements(input, 1))
}

pub fn part2() {
    let input = std::str::from_utf8(include_bytes!("../../resource/day12/input1")).unwrap();
    println!("{}", hot_springs::count_arrangements(input, 5))
}