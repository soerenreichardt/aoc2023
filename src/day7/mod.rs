mod camel_cards;

pub fn part1() {
    let input = std::str::from_utf8(include_bytes!("../../resource/day7/input1")).unwrap();
    println!("{}", camel_cards::score_hands(input));
}