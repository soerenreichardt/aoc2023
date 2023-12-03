use std::collections::HashMap;

pub(crate) fn fewest_cubes(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line
            .trim()
            .strip_prefix("Game ").expect("Invalid line format")
            .split_once(':')
            .map(|(line_num, content)| (line_num.parse::<u32>().unwrap(), content))
            .expect("Invalid line numbering"))
        .filter_map(|(_, content)| valid_game(content))
        .sum()
}

fn valid_game(line: &str) -> Option<u32> {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    for round in line.split(';').map(|round| round.trim()) {
        let mut color_counts = HashMap::new();
        for cube_set in round.split(',').map(|cube_set| cube_set.trim()) {
            let (num, color) = cube_set.split_once(' ').expect("Invalid cube set");
            let num = num.parse::<u32>().unwrap();
            let entry = color_counts.entry(color).or_insert(0u32);
            *entry += num;
        }
        let red = *color_counts.entry("red").or_insert(0);
        let green = *color_counts.entry("green").or_insert(0);
        let blue = *color_counts.entry("blue").or_insert(0);

        max_red = max_red.max(red);
        max_green = max_green.max(green);
        max_blue = max_blue.max(blue);
    }
    Some(max_blue * max_green * max_red)
}

#[cfg(test)]
mod tests {
    use crate::day2::cubes::fewest_cubes;

    #[test]
    fn should_check_fewest_cubes() {
        let test_input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        assert_eq!(fewest_cubes(test_input), 2286);
    }
}