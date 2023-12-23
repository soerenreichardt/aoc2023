use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::{Hash, Hasher};

pub fn least_heat_loss(input: &str) -> usize {
    let city_blocks = input
        .lines()
        .map(|line| line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    restricted_dijksta(&city_blocks, (0, 0), (city_blocks.len() - 1, city_blocks[0].len() - 1))
}

fn restricted_dijksta(city_blocks: &[Vec<u32>], start: (usize, usize), end: (usize, usize)) -> usize {
    let height = city_blocks.len();
    let width = city_blocks[0].len();

    let mut priority_queue = BinaryHeap::new();
    let mut heat_losses = HashMap::new();
    priority_queue.push(Node {
        position: start,
        heat_loss: 0,
        direction: Direction::Right,
        predecessor: None,
    });

    while let Some(node) = priority_queue.pop() {
        if node.position == end {
            print_predecessors(&node);
            return node.heat_loss;
        }

        if node.heat_loss > *heat_losses.entry(node.clone()).or_insert(usize::MAX) {
            continue;
        }

        for (neighbor_position, direction) in compute_neighbor_positions(node.position.clone(), height, width) {
            if node.direction.is_reverse(&direction) {
                // println!("does this happen?");
                continue;
            }
            let heat_loss = node.heat_loss + (city_blocks[neighbor_position.0][neighbor_position.1] as usize);
            let next_node = Node { position: neighbor_position, heat_loss, direction, predecessor: Some(Box::new(node.clone())) };
            if heat_loss < *heat_losses.entry(next_node.clone()).or_insert(usize::MAX) && valid_straight_line_length(&next_node) {
                heat_losses.insert(next_node.clone(), next_node.heat_loss);
                priority_queue.push(next_node);
            }
        }
    }

    panic!("No path found");
}

fn compute_neighbor_positions(position: (usize, usize), height: usize, width: usize) -> Vec<((usize, usize), Direction)> {
    let mut neighbors = Vec::new();
    if position.0 > 0 {
        neighbors.push(((position.0 - 1, position.1), Direction::Up));
    }
    if position.0 < height - 1 {
        neighbors.push(((position.0 + 1, position.1), Direction::Down));
    }
    if position.1 > 0 {
        neighbors.push(((position.0, position.1 - 1), Direction::Left));
    }
    if position.1 < width - 1 {
        neighbors.push(((position.0, position.1 + 1), Direction::Right));
    }
    neighbors
}

fn valid_straight_line_length(node: &Node) -> bool {
    let node_position = node.position;
    let mut current_node = node;
    let mut predecessor_position = node_position.clone();
    let direction = &node.direction;
    for _ in 0..3 {
        match &current_node.predecessor {
            Some(predecessor) => {
                predecessor_position = predecessor.position;
                current_node = predecessor.as_ref();
                if direction != &predecessor.direction {
                    return true;
                }
            },
            None => return true
        }
    }

    false
}

fn print_predecessors(node: &Node) {
    let mut current_node = node;
    println!("{:?}", current_node.position);
    while let Some(predecessor) = &current_node.predecessor {
        println!("{:?} {:?}", predecessor.position, predecessor.direction);
        current_node = predecessor.as_ref();
    }
}

#[derive(Clone, Debug, Eq)]
struct Node {
    position: (usize, usize),
    heat_loss: usize,
    direction: Direction,
    predecessor: Option<Box<Node>>,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn is_reverse(&self, other: &Direction) -> bool {
        match self {
            Direction::Up => other == &Direction::Down,
            Direction::Down => other == &Direction::Up,
            Direction::Left => other == &Direction::Right,
            Direction::Right => other == &Direction::Left,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.position.eq(&other.position)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        // self.direction.hash(state);
        // let in_dir = self.predecessor.as_ref().map(|predecessor| predecessor.direction.clone()).unwrap_or(Direction::Right);
        // in_dir.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_least_heat_loss() {
        let input = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;
        assert_eq!(least_heat_loss(input), 102);
    }

    #[test]
    fn test_least_heat_loss_2() {
        let input = r#"123
456
789"#;
        assert_eq!(least_heat_loss(input), 102);
    }
}