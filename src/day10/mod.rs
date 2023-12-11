mod pipe_maze;

pub fn part1() {
    let input = std::str::from_utf8(include_bytes!("../../resource/day10/input1")).unwrap();
    println!("{}", pipe_maze::farthest_point_in_loop(input))
}

pub fn part2() {
    let input = std::str::from_utf8(include_bytes!("../../resource/day10/input1")).unwrap();
    println!("{}", pipe_maze::enclosed_tiles(input))
}