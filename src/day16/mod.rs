mod the_floor_will_be_lava;

pub fn part1() {
    let input = std::str::from_utf8(include_bytes!("../../resource/day16/input1")).unwrap();
    println!("{}", the_floor_will_be_lava::energized_tiles(input))
}
