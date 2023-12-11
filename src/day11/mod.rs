mod cosmic_expansion;

pub fn part1() {
    let input = std::str::from_utf8(include_bytes!("../../resource/day11/input1")).unwrap();
    println!("{}", crate::day11::cosmic_expansion::all_pair_shortest_path(input))
}
