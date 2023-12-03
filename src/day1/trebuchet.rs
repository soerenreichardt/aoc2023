#[derive(Default)]
struct DigitTracker {
    first: Option<u32>,
    last: Option<u32>
}

impl DigitTracker {
    fn process_char(&mut self, ch: char) {
        if ch.is_digit(10) {
            let digit = ch.to_digit(10).unwrap();
            self.set_digit(digit);
        }
    }

    fn combine_digits(&self) -> u32 {
        match (self.first, self.last) {
            (Some(first), Some(last)) => first * 10 + last,
            _ => panic!("First or last value is not set")
        }
    }

    fn set_digit(&mut self, digit: u32) {
        if self.first.is_none() {
            self.first = Some(digit);
        }
        self.last = Some(digit);
    }
}

pub fn process_input(input: &str) -> u32 {
    input
        .lines()
        .into_iter()
        .map(|line| line.trim())
        .map(replace_number_words)
        .map(sum_first_and_last_digit)
        .sum()
}

fn sum_first_and_last_digit(line: String) -> u32 {
    let mut digit_tracker = DigitTracker::default();
    for ch in line.chars().into_iter() {
        digit_tracker.process_char(ch);
    }

    digit_tracker.combine_digits()
}

fn replace_number_words(line: &str) -> String {
    let mut curated_line: Vec<u8> = Vec::new();
    let mut index = 0;
    while index < line.len() {
        let line_slice = &line[index..];
        if line_slice.starts_with("one") {
            curated_line.push('1' as u8);
        } else if line_slice.starts_with("two") {
            curated_line.push('2' as u8);
        } else if line_slice.starts_with("three") {
            curated_line.push('3' as u8);
        } else if line_slice.starts_with("four") {
            curated_line.push('4' as u8);
        } else if line_slice.starts_with("five") {
            curated_line.push('5' as u8);
        } else if line_slice.starts_with("six") {
            curated_line.push('6' as u8);
        } else if line_slice.starts_with("seven") {
            curated_line.push('7' as u8);
        } else if line_slice.starts_with("eight") {
            curated_line.push('8' as u8);
        } else if line_slice.starts_with("nine") {
            curated_line.push('9' as u8);
        } else {
            curated_line.push(line_slice.chars().next().unwrap() as u8)
        }
        index += 1;
    }

    String::from_utf8(curated_line).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day1::trebuchet::replace_number_words;

    #[test]
    fn should_curate_line() {
        assert_eq!(replace_number_words("two1nine"), "2wo19ine")
    }
}