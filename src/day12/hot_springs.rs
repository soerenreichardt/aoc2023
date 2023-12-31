use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;
use std::str::FromStr;

impl From<char> for SpringState {
    fn from(c: char) -> Self {
        match c {
            '.' => SpringState::Operational,
            '#' => SpringState::Damaged,
            '?' => SpringState::Unknown,
            _ => panic!("Invalid spring state")
        }
    }
}

impl From<(&str, usize)> for SpringRecord {

    fn from((s, copies): (&str, usize)) -> Self {
        let (row, damage_group) = s.split_once(' ').unwrap();
        let mut state_row: Vec<_> = row.chars().map(|c| c.into()).collect();
        let mut damage_groups: Vec<_> = damage_group.split(',').map(|s| s.parse::<usize>().unwrap()).collect();

        let state_row_copy = state_row.clone();
        let damage_groups_copy = damage_groups.clone();

        (0..copies - 1).for_each(|_| {
            state_row.push(SpringState::Unknown);
            state_row.extend(state_row_copy.clone());
            damage_groups.extend(damage_groups_copy.clone());
        });
        SpringRecord { state_row, damage_groups, copies }
    }
}

impl Display for SpringState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SpringState::Operational => write!(f, "."),
            SpringState::Damaged => write!(f, "#"),
            SpringState::Unknown => write!(f, "?"),
        }
    }
}

impl Debug for SpringState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl Display for SpringRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for state in &self.state_row {
            write!(f, "{}", state)?;
        }
        write!(f, " ")?;
        for damage_group in &self.damage_groups {
            write!(f, "{},", damage_group)?;
        }
        Ok(())
    }
}

pub fn count_arrangements(input: &str, copies: usize) -> usize {
    let spring_records = parse_input(input, copies);
    spring_records.iter()
        .map(|record| record.find_valid_replacements_for_unknown())
        .sum()
}

fn parse_input(input: &str, copies: usize) -> Vec<SpringRecord> {
    input.lines().map(|line| SpringRecord::from((line, copies))).collect()
}

impl SpringRecord {
    fn find_valid_replacements_for_unknown(&self) -> usize {
        let damage_tracker = DamageTracker { damage_group_index: 0, damage_groups: &self.damage_groups, damage_counter: 0, next_can_be_damaged: true };
        let mut cache = HashMap::new();
        Self::fork_replacement_search(damage_tracker, self.state_row.clone(), 0, &mut cache)
    }

    fn fork_replacement_search(
        mut damage_tracker: DamageTracker,
        mut state_row: Vec<SpringState>,
        state_row_start: usize,
        cache: &mut HashMap<TraversalState, usize>
    ) -> usize {

        for (col, spring_state) in state_row.iter_mut().enumerate().skip(state_row_start) {

            let valid_state = match spring_state {
                SpringState::Operational => damage_tracker.allow_next_to_be_damaged(),
                SpringState::Damaged => damage_tracker.record_damage(),
                SpringState::Unknown => match damage_tracker.resolve_unknown_spring_state() {
                    SpringState::Operational => {
                        *spring_state = SpringState::Operational;
                        damage_tracker.allow_next_to_be_damaged()
                    },
                    SpringState::Damaged => {
                        *spring_state = SpringState::Damaged;
                        damage_tracker.record_damage()
                    },
                    SpringState::Unknown => {
                        let key = TraversalState::new(damage_tracker.damage_group_index, col, damage_tracker.damage_counter);
                        if let Some(entry) = cache.get(&key) {
                            return *entry;
                        }
                        // this can be either Operational or Damaged
                        let mut forked_state_row = state_row.clone();
                        forked_state_row[col] = SpringState::Damaged;
                        state_row[col] = SpringState::Operational;
                        let fork_result = Self::fork_replacement_search(damage_tracker.clone(), forked_state_row, col, cache) +
                            Self::fork_replacement_search(damage_tracker.clone(), state_row, col, cache);

                        cache.insert(key, fork_result);
                        return fork_result;

                    }
                }
            };
            if !valid_state {
                return 0;
            }
        }

        if damage_tracker.is_exhausted() {
            return 1;
        }
        0
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct TraversalState {
    normalized_damage_tracker_index: usize,
    normalized_col: usize,
    damage_counter: usize
}

impl TraversalState {
    fn new(damage_tracker_index: usize, col: usize, damage_counter: usize) -> Self {
        TraversalState { normalized_damage_tracker_index: damage_tracker_index, normalized_col: col, damage_counter }
    }
}

#[derive(Clone, Debug)]
struct DamageTracker<'a> {
    damage_group_index: usize,
    damage_groups: &'a [usize],
    damage_counter: usize,
    next_can_be_damaged: bool
}

impl<'a> DamageTracker<'a> {
    fn current_damage(&mut self) -> usize {
        match self.peek() {
            Some(damage_group) => damage_group,
            None => panic!("No more damage groups")
        }
    }

    fn allow_next_to_be_damaged(&mut self) -> bool {
        if self.damage_group_done() {
            self.next_can_be_damaged = true;
            return true
        }
        false
    }

    fn record_damage(&mut self) -> bool {
        if self.peek().is_none() || !self.next_can_be_damaged {
            return false;
        }
        self.damage_counter += 1;
        if self.damage_counter == self.current_damage() {
            self.next_can_be_damaged = false;
            self.next();
            self.damage_counter = 0;
        }
        true
    }

    fn next_can_be_damaged(&self) -> bool {
        self.next_can_be_damaged
    }

    fn damage_group_done(&self) -> bool {
        self.damage_counter == 0
    }

    fn resolve_unknown_spring_state(&mut self) -> SpringState {
        if !self.next_can_be_damaged() {
            SpringState::Operational
        } else if self.damage_counter > 0 {
            SpringState::Damaged
        } else {
            SpringState::Unknown
        }
    }

    fn is_exhausted(&self) -> bool {
        self.peek().is_none() && self.damage_counter == 0
    }

    fn peek(&self) -> Option<usize> {
        if self.damage_group_index >= self.damage_groups.len() {
            return None;
        }
        Some(self.damage_groups[self.damage_group_index])
    }

    fn next(&mut self) {
        self.damage_group_index += 1
    }
}



#[derive(Debug)]
struct SpringRecord {
    state_row: Vec<SpringState>,
    damage_groups: Vec<usize>,
    copies: usize
}

#[derive(Clone, PartialEq)]
enum SpringState {
    Operational,
    Damaged,
    Unknown
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_count_arrangements() {
        let input = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;
        assert_eq!(count_arrangements(input, 1), 21);
    }

    #[test]
    fn should_count_arrangements_times_5() {
        let input = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;
        assert_eq!(count_arrangements(input, 5), 525152);
    }
}