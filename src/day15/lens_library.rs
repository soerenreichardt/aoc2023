pub fn sum_hash_values(input: &str) -> usize {
    input.split(',').map(compute_hash).sum()
}

fn compute_hash(step: &str) -> usize {
    step.chars().fold(0, |acc, c| {
        if c == '\n' {
            return acc;
        }
        ((acc + ((c as u32) as u64)) * 17) % 256
    }) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_sum_hashes() {
        let input = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;
        assert_eq!(sum_hash_values(input), 1320)
    }
}