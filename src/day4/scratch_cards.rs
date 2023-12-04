use std::collections::HashSet;

pub(crate) fn winning_points(input: &str) -> u32 {
    preprocess_input(input)
        .into_iter()
        .map(|(win_numbers, game)| score_scratch_card(win_numbers, game))
        .sum()
}

pub(crate) fn count_games(input: &str) -> u32 {
    count_cards_with_copies(preprocess_input(input))
}

fn preprocess_input(input: &str) -> Vec<(HashSet<u32>, Vec<u32>)> {
    input
        .lines()
        .map(|line| line.split_once(':').expect("Invalid line format").1)
        .map(|line| line.trim().split_once('|').expect("Invalid line format"))
        .map(|(win_numbers, game)| (number_str_to_set(win_numbers), number_str_to_vec(game)))
        .collect::<Vec<_>>()
}

fn number_str_to_set(number_str: &str) -> HashSet<u32> {
    number_str.split_ascii_whitespace().fold(HashSet::new(), |mut numbers, s| {
        numbers.insert(s.parse::<u32>().unwrap());
        numbers
    })
}

fn number_str_to_vec(number_str: &str) -> Vec<u32> {
    number_str.split_ascii_whitespace().fold(Vec::new(), |mut numbers, s| {
        numbers.push(s.parse::<u32>().unwrap());
        numbers
    })
}

fn score_scratch_card(winning_numbers: HashSet<u32>, game_numbers: Vec<u32>) -> u32 {
    let mut score = 0;

    for num in game_numbers {
        if winning_numbers.contains(&num) {
            if score == 0 {
                score = 1;
            } else {
                score <<= 1;
            }
        }
    }

    score
}

fn count_cards_with_copies(games: Vec<(HashSet<u32>, Vec<u32>)>) -> u32 {
    let mut card_stack = games.iter().enumerate().collect::<Vec<_>>();
    let mut game_counter = 0;

    while !card_stack.is_empty() {
        let mut copies = Vec::new();
        for (game_num, (winning_numbers, game_numbers)) in card_stack.into_iter() {
            game_counter += 1;
            let mut copy_counter = 0;
            for num in game_numbers {
                if winning_numbers.contains(&num) {
                    copy_counter += 1;
                }
            }

            for copy_index in 1..=copy_counter {
                let copy_game_num = game_num + copy_index;
                copies.push((copy_game_num, &games[copy_game_num]))
            }
        }
        // add copies to stack
        card_stack = copies;
    }

    game_counter
}

#[cfg(test)]
mod tests {
    use crate::day4::scratch_cards::{count_games, winning_points};

    #[test]
    fn should_score_cards() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        assert_eq!(winning_points(input), 13);
    }

    #[test]
    fn should_count_games() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        assert_eq!(count_games(input), 30);
    }
}