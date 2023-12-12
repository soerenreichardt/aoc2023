mod cosmic_expansion;

pub fn part1() {
    let input = std::str::from_utf8(include_bytes!("../../resource/day11/input1")).unwrap();
    println!("{}", crate::day11::cosmic_expansion::all_pairs_shortest_path(input))
}

pub fn part2() {
    let input = std::str::from_utf8(include_bytes!("../../resource/day11/input1")).unwrap();
    println!("{}", crate::day11::cosmic_expansion::all_pairs_million_times_expand_shortest_path(input))
}
