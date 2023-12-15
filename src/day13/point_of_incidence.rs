pub fn reflecting_notes_count(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|block| block
            .lines()
            .map(|line| line
                .chars()
                .collect::<Vec<_>>())
            .collect::<Vec<_>>()
        )
        .map(find_reflection_in_pattern)
        .sum()
}

fn find_reflection_in_pattern(pattern: Vec<Vec<char>>) -> usize {
    // vertical reflections
    let vertical_match = pattern[0].windows(2).enumerate()
        .map(|(start, _)| start)
        .filter(|candidate| check_vertical_reflection_candidate(&pattern, *candidate))
        .next();

    let horizontal_match = pattern.windows(2).enumerate()
        .map(|(start, _)| start)
        .filter(|candidate| check_horizontal_reflection_candidate(&pattern, *candidate))
        .next();

    match (vertical_match, horizontal_match) {
        (Some(vertical), None) => vertical + 1,
        (None, Some(horizontal)) => 100 * (horizontal + 1),
        _ => panic!("No reflection found")
    }
}

fn check_horizontal_reflection_candidate(pattern: &Vec<Vec<char>>, start: usize) -> bool {
    let start = start + 1;
    let check_length = usize::min(pattern.len() - start, start);
    (start..(start + check_length)).fold(0, |wrong_matches, row_num| {
        wrong_matches + (0..pattern[0].len()).fold(0, |wrong_matches, col_num| {
            if pattern[row_num][col_num] == pattern[(start - 1) - (row_num - start)][col_num] {
                wrong_matches
            } else {
                wrong_matches + 1
            }
        })
    }) == 1
}

fn check_vertical_reflection_candidate(pattern: &Vec<Vec<char>>, start: usize) -> bool {
    let start = start + 1;
    let check_length = usize::min(pattern[0].len() - start, start);
    (start..(start + check_length)).fold(0, |wrong_matches, col_num| {
        wrong_matches + pattern.iter().fold(0, |wrong_matches, row| {
            if row[col_num] == row[(start - 1) - (col_num - start)] {
                wrong_matches
            } else {
                wrong_matches + 1
            }
        })
    }) == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_vertical_reflections() {
        let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."#;
        assert_eq!(reflecting_notes_count(input), 300);
    }

    #[test]
    fn should_find_horizontal_reflections() {
        let input = r#"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;
        assert_eq!(reflecting_notes_count(input), 100);
    }
}
