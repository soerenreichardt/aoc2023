
pub fn won_races(input: &str) -> u64 {
    let input = input.lines().collect::<Vec<_>>();
    let time = input[0]
        .split_once(':')
        .unwrap().1
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let distance = input[1]
        .split_once(':')
        .unwrap().1
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let races = time.into_iter().zip(distance).collect::<Vec<_>>();
    find_won_races(&races)
}

pub fn won_races_part2(input: &str) -> u64 {
    let input = input.lines().collect::<Vec<_>>();
    let time = input[0]
        .split_once(':')
        .unwrap().1
        .split_ascii_whitespace()
        .collect::<Vec<_>>()
        .concat()
        .parse::<u64>().unwrap();
    let distance = input[1]
        .split_once(':')
        .unwrap().1
        .split_ascii_whitespace()
        .collect::<Vec<_>>()
        .concat()
        .parse::<u64>().unwrap();

    find_won_races(&vec![(time, distance)])
}

fn find_won_races(races: &[(u64, u64)]) -> u64 {
    let mut result = 1;

    for (time, distance) in races {
        let mut race_winning_charges = 0;
        for charge in 0..=*time {
            let remaining_time_after_charge = time - charge;
            let race_distance = charge * remaining_time_after_charge;
            if race_distance > *distance {
                race_winning_charges += 1;
            }
        }

        if race_winning_charges > 0 {
            // let race_winning_charges = if race_winning_charges % 2 == 0 { race_winning_charges * 2 } else { (race_winning_charges - 1) * 2 + 1 };
            result *= race_winning_charges
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::day6::race::won_races;

    #[test]
    fn should_count_won_races() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
        assert_eq!(won_races(input), 288)
    }
}