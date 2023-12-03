#[derive(Debug, PartialEq)]
struct NumberSpan {
    line: usize,
    start: usize,
    end: usize,
    value: u32,
    status: Status,
}

#[derive(Debug, PartialEq)]
enum Status {
    Created,
    NumberSet,
    Complete,
}

#[derive(Debug, PartialEq)]
struct Symbol {
    line: usize,
    position: usize,
}

impl NumberSpan {
    fn new(line_num: usize) -> Self {
        NumberSpan {
            line: line_num,
            start: 0,
            end: 0,
            value: 0,
            status: Status::Created,
        }
    }

    fn set_position(&mut self, position: usize) {
        match self.status {
            Status::Created => {
                self.start = position;
                self.end = position;
                self.status = Status::NumberSet;
            }
            Status::NumberSet => {
                self.end = position;
            }
            Status::Complete => panic!("Span already complete")
        }
    }

    fn finish(&mut self, line: &str) {
        match self.status {
            Status::NumberSet => match line[self.start..self.end + 1].parse::<u32>() {
                Ok(value) => self.value = value,
                Err(_) => panic!("{}", line[self.start..self.end + 1].to_string())
            }
            Status::Complete => panic!("Span already complete"),
            Status::Created => return
        }
        self.status = Status::Complete
    }

    fn is_complete(&self) -> bool {
        self.status == Status::Complete
    }
}

impl Symbol {
    fn is_adjacent_to(&self, span: &NumberSpan) -> bool {
        let line_diff = self.line.abs_diff(span.line);
        if !(line_diff == 0 || line_diff == 1) {
            return false;
        }

        let start_diff = self.position.abs_diff(span.start);
        let end_diff = self.position.abs_diff(span.end);

        if !(start_diff == 0 || start_diff == 1) && !(end_diff == 0 || end_diff == 1) {
            return false;
        }

        true
    }
}

pub(crate) fn gear_ratio(input: &str) -> u32 {
    input
        .lines().enumerate()
        .map(|(line_num, line)| process_line(line.trim(), line_num))
        .reduce(|(mut lhs_span, mut lhs_symbol), (rhs_span, rhs_symbol)| {
            lhs_span.extend(rhs_span);
            lhs_symbol.extend(rhs_symbol);
            (lhs_span, lhs_symbol)
        })
        .into_iter()
        .flat_map(|(spans, symbols)| filter_number_spans(symbols, spans))
        .sum()
}

fn process_line(line: &str, line_num: usize) -> (Vec<NumberSpan>, Vec<Symbol>) {
    let mut symbols = Vec::new();
    let mut number_spans = Vec::new();
    let mut active_span = NumberSpan::new(line_num);
    for (position, ch) in line.chars().enumerate() {
        match ch {
            '0'..='9' => active_span.set_position(position),
            '.' => active_span.finish(line),
            '*' => {
                symbols.push(Symbol { line: line_num, position });
                active_span.finish(line)
            }
            _ => ()
        }

        if active_span.is_complete() {
            number_spans.push(active_span);
            active_span = NumberSpan::new(line_num);
        }
    }

    // finish span if no '.' comes last in line
    active_span.finish(line);
    if active_span.is_complete() {
        number_spans.push(active_span);
    }

    (number_spans, symbols)
}

fn filter_number_spans(symbols: Vec<Symbol>, spans: Vec<NumberSpan>) -> Vec<u32> {
    let mut adjacent_number = Vec::new();
    symbols
        .iter()
        .map(|symbol| spans.iter().fold(Vec::new(), |mut adj_numbers: Vec<u32>, span| {
            if symbol.is_adjacent_to(span) {
                adj_numbers.push(span.value);
            }
            adj_numbers
        }))
        .filter(|adj_numbers| adj_numbers.len() == 2)
        .for_each(|adj_numbers| adjacent_number.push(adj_numbers[0] * adj_numbers[1]));
    adjacent_number
}

#[cfg(test)]
mod tests {
    use crate::day3::gears::{gear_ratio, NumberSpan, process_line, Status, Symbol};

    #[test]
    fn should_count_adjacent_numbers() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        assert_eq!(gear_ratio(input), 467835);
    }

    #[test]
    fn should_compute_spans_and_symbols() {
        let (spans, symbols) = process_line("467.#...11", 42);
        assert_eq!(spans.len(), 2);
        assert_eq!(symbols.len(), 1);

        assert_eq!(spans[0], NumberSpan { line: 42, start: 0, end: 2, value: 467, status: Status::Complete });
        assert_eq!(spans[1], NumberSpan { line: 42, start: 8, end: 9, value: 11, status: Status::Complete });

        assert_eq!(symbols[0], Symbol { line: 42, position: 4 })
    }
}