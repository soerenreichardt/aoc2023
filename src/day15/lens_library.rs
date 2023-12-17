pub fn sum_hash_values(input: &str) -> usize {
    input.split(',').map(compute_hash).sum()
}

pub fn place_lenses(input: &str) -> usize {
    let mut boxes: Boxes = Boxes(vec![Vec::new(); 256].try_into().unwrap());
    parse_instructions(input)
        .into_iter()
        .for_each(|instruction| boxes.execute_instruction(instruction));
    boxes.focusing_power()
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.split(',').map(|s| if s.contains('-') {
        let label = &s[..s.len() - 1];
        (label, Operation::Remove)
    } else if s.contains('=') {
        let (label, focal_length) = s.split_once('=').unwrap();
        (label, Operation::Replace(focal_length.parse::<u8>().unwrap()))
    } else {
        panic!("Invalid input")
    })
        .map(|(label, operation)| Instruction::new(label, operation))
        .collect::<Vec<_>>()
}

struct Instruction<'a> {
    label: &'a str,
    operation: Operation,
}

enum Operation {
    Remove,
    Replace(u8),
}

#[derive(Debug)]
struct Boxes<'a>([Vec<Lens<'a>>; 256]);

#[derive(Clone, Debug)]
struct Lens<'a> {
    label: &'a str,
    focal_length: u8,
}

impl<'a> Instruction<'a> {
    fn new(label: &'a str, operation: Operation) -> Self {
        Self {
            label,
            operation,
        }
    }
}

impl<'a> Boxes<'a> {
    fn execute_instruction(&mut self, instruction: Instruction<'a>) {
        let lens_label_hash = compute_hash(instruction.label);
        let selected_box = &mut self.0[lens_label_hash];
        match instruction.operation {
            Operation::Remove => {
                find_lens_position(selected_box, instruction.label.clone())
                    .map(|lens_position| selected_box.remove(lens_position));
            }
            Operation::Replace(focal_length) => {
                let lens = Lens { label: instruction.label.clone(), focal_length };
                match find_lens_position(selected_box, &instruction.label) {
                    Some(index) => { selected_box[index] = lens; }
                    None => { selected_box.push(lens.clone()); }
                }
            }
        }
    }

    fn focusing_power(&self) -> usize{
        let mut focusing_power = 0;
        for (box_num, selected_box) in self.0.iter().enumerate() {
            for (slot, lens) in selected_box.iter().enumerate() {
                focusing_power += (box_num + 1) * (slot + 1) * lens.focal_length as usize;
            }
        }
        focusing_power
    }
}

fn compute_hash(step: &str) -> usize {
    step.chars().fold(0, |acc, c| {
        if c == '\n' {
            return acc;
        }
        ((acc + ((c as u32) as u64)) * 17) % 256
    }) as usize
}

fn find_lens_position<'a>(selected_box: &'a Vec<Lens<'a>>, label: &'a str) -> Option<usize> {
    selected_box
        .iter()
        .enumerate()
        .find_map(|(pos, lens)| if lens.label == label {
            Some(pos)
        } else {
            None
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_sum_hashes() {
        let input = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;
        assert_eq!(sum_hash_values(input), 1320)
    }

    #[test]
    fn should_place_lenses() {
        let input = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;
        assert_eq!(place_lenses(input), 145);
    }
}