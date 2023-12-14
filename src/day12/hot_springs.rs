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

impl FromStr for SpringRecord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (row, damage_group) = s.split_once(' ').unwrap();
        let state_row = row.chars().map(|c| c.into()).collect();
        let damage_groups = damage_group.split(',').map(|s| s.parse::<usize>().unwrap()).collect();
        Ok(SpringRecord { state_row, damage_groups })
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

pub fn count_arrangements(input: &str) -> u32 {
    let spring_records = parse_input(input);
    spring_records.iter()
        .map(|record| record.find_valid_replacements_for_unknown().len() as u32)
        .sum()
}

fn parse_input(input: &str) -> Vec<SpringRecord> {
    input.lines().map(|line| SpringRecord::from_str(line).unwrap()).collect()
}

impl SpringRecord {
    fn find_valid_replacements_for_unknown(&self) -> Vec<Vec<SpringState>> {
        let damage_groups = self.damage_groups.iter().peekable();
        let damage_tracker = DamageTracker { damage_groups, damage_counter: 0, next_can_be_damaged: true };
        println!("========================================");
        println!("start  {:?} - {:?}", self.state_row, self.damage_groups);
        Self::fork_replacement_search(damage_tracker, self.state_row.clone(), 0)
    }

    fn fork_replacement_search(
        mut damage_tracker: DamageTracker,
        mut state_row: Vec<SpringState>,
        state_row_start: usize
    ) -> Vec<Vec<SpringState>> {
        let mut replacements = Vec::new();

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
                        // this can be either Operational or Damaged
                        let mut forked_state_row = state_row.clone();
                        forked_state_row[col] = SpringState::Damaged;
                        replacements.extend(Self::fork_replacement_search(damage_tracker.clone(), forked_state_row, col));

                        state_row[col] = SpringState::Operational;
                        replacements.extend(Self::fork_replacement_search(damage_tracker, state_row, col));
                        return replacements;
                    }
                }
            };
            if !valid_state {
                return vec![];
            }
        }

        if damage_tracker.is_exhausted() {
            println!("result {:?}", state_row);
            replacements.push(state_row);
        }
        replacements
    }
}

#[derive(Clone, Debug)]
struct DamageTracker<'a> {
    damage_groups: Peekable<Iter<'a, usize>>,
    damage_counter: usize,
    next_can_be_damaged: bool
}

impl<'a> DamageTracker<'a> {
    fn current_damage(&mut self) -> usize {
        match self.damage_groups.peek() {
            Some(damage_group) => **damage_group,
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
        if self.damage_groups.peek().is_none() || !self.next_can_be_damaged {
            return false;
        }
        self.damage_counter += 1;
        if self.damage_counter == self.current_damage() {
            self.next_can_be_damaged = false;
            self.damage_groups.next();
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

    fn is_exhausted(&mut self) -> bool {
        self.damage_groups.peek().is_none() && self.damage_counter == 0
    }
}



#[derive(Debug)]
struct SpringRecord {
    state_row: Vec<SpringState>,
    damage_groups: Vec<usize>
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
        assert_eq!(count_arrangements(input), 21);
    }
}