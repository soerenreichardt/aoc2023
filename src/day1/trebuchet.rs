#[derive(Default)]
struct DigitTracker {
    first: Option<u32>,
    last: Option<u32>
}

impl DigitTracker {
    fn process_char(&mut self, ch: char) {
        if ch.is_digit(10) {
            let digit = ch.to_digit(10).unwrap();
            if self.first.is_none() {
                self.first = Some(digit);
            }
            self.last = Some(digit);
        }
    }

    fn combine_digits(&self) -> u32 {
        match (self.first, self.last) {
            (Some(first), Some(last)) => first * 10 + last,
            _ => panic!("First or last value is not set")
        }
    }
}

pub fn process_input(input: &str) -> u32 {
    input
        .lines()
        .into_iter()
        .map(|line| line.trim())
        .map(|line| sum_first_and_last_digit(line))
        .sum()
}

fn sum_first_and_last_digit(line: &str) -> u32 {
    let mut digit_tracker = DigitTracker::default();
    for ch in line.chars().into_iter() {
        digit_tracker.process_char(ch);
    }

    digit_tracker.combine_digits()
}