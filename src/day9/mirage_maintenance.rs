pub fn extrapolate(input: &str) -> i64 {
    input.lines()
        .map(|line| line.split_ascii_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>())
        .map(extrapolate_from_history)
        .sum()
}

pub fn extrapolate_backwards(input: &str) -> i64 {
    input.lines()
        .map(|line| line.split_ascii_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>())
        .map(extrapolate_backwards_from_history)
        .sum()
}

fn extrapolate_from_history(history: Vec<i64>) -> i64 {
    let all_deltas = compute_deltas_until_all_zero(history);
    extrapolate_from_deltas(all_deltas)
}

fn extrapolate_backwards_from_history(history: Vec<i64>) -> i64 {
    let all_deltas = compute_deltas_until_all_zero(history);
    extrapolate_backwards_from_deltas(all_deltas)
}

fn extrapolate_from_deltas(all_deltas: Vec<Vec<i64>>) -> i64 {
    let mut extrapolation = 0;

    for deltas in all_deltas[..all_deltas.len()-1].iter().rev() {
        extrapolation += deltas.last().unwrap();
    }

    extrapolation
}

fn extrapolate_backwards_from_deltas(all_deltas: Vec<Vec<i64>>) -> i64 {
    let mut extrapolation = 0;

    for deltas in all_deltas[..all_deltas.len() - 1].iter().rev() {
        extrapolation = deltas.first().unwrap() - extrapolation;
    }

    println!("{}", extrapolation);
    extrapolation
}

fn compute_deltas_until_all_zero(history: Vec<i64>) -> Vec<Vec<i64>> {
    let mut deltas = Vec::new();
    deltas.push(history);

    while !deltas.last().unwrap().iter().all(|delta| *delta == 0) {
        deltas.push(compute_deltas(deltas.last().unwrap()))
    }

    deltas
}

fn compute_deltas(history: &[i64]) -> Vec<i64> {
    history.windows(2).map(|slice| slice[1] - slice[0]).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use crate::day9::mirage_maintenance::{compute_deltas, extrapolate, extrapolate_backwards};

    #[test]
    fn should_extrapolate() {
        let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
        assert_eq!(extrapolate(input), 114);
    }

    #[test]
    fn should_extrapolate_backwards() {
        let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
        assert_eq!(extrapolate_backwards(input), 2);
    }

    #[test]
    fn should_compute_deltas() {
        let input: Vec<i64> = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(compute_deltas(&input), vec![2, 3, 4, 5, 6]);
    }
}