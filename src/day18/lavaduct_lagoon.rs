use std::fmt::Debug;
use std::str::FromStr;

use crate::day18::polygon::{Polygon, Segment, SegmentKind};

#[derive(Debug)]
pub(crate) struct DigPlan<'a> {
    pub(crate) instructions: Vec<Instruction<'a>>,
}

#[derive(Debug)]
pub(crate) struct Instruction<'a> {
    pub(crate) direction: Direction,
    pub(crate) amount: usize,
    pub(crate) color: &'a str,
}

#[derive(Debug)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Bounds {
    min_row: i64,
    max_row: i64,
    min_col: i64,
    max_col: i64,
}

struct SortedPolygon {
    horizontal_segments: Vec<Segment>,
    vertical_segments: Vec<Segment>,
}

pub fn cubic_meters_of_lava(input: &str) -> usize {
    let dig_plan = DigPlan::from(input);
    let polygon = Polygon::from_gid_plan(dig_plan);
    let polygon = SortedPolygon::from_polygon(polygon);
    enclosed_tiles(&polygon)
}

fn enclosed_tiles(polygon: &SortedPolygon) -> usize {
    let bounds = polygon.calculate_bounds();
    let mut result = 0;
    for row in bounds.min_row..=bounds.max_row {
        let filtered_segments = polygon.vertical_segments
            .iter()
            .filter(|segment| row >= segment.start.row && row <= segment.end.row)
            .collect::<Vec<_>>();
        result += filtered_segments
            .chunks(2)
            .map(|pair| {
                println!("[{}] {:?}", row, pair);
                i64::abs(pair[1].start.col - pair[0].start.col)
            })
            .into_iter()
            // .collect::<Vec<_>>()
            .sum::<i64>() as usize;
    }

    result
}

impl SortedPolygon {
    fn from_polygon(polygon: Polygon) -> Self {
        let mut horizontal_segments = Vec::new();
        let mut vertical_segments = Vec::new();
        for segment in polygon.segments {
            match segment.kind {
                SegmentKind::Horizontal => horizontal_segments.push(segment),
                SegmentKind::Vertical => vertical_segments.push(segment)
            }
        }

        horizontal_segments.sort_by(|a, b| a.start.col.cmp(&b.start.col));
        vertical_segments.sort_by(|a, b| a.start.row.cmp(&b.start.row).then(a.start.col.cmp(&b.start.col)));

        SortedPolygon { horizontal_segments, vertical_segments }
    }

    fn calculate_bounds(&self) -> Bounds {
        let min_row = self.horizontal_segments.first().unwrap().start.row;
        let max_row = self.horizontal_segments.last().unwrap().start.row;
        let min_col = self.vertical_segments.first().unwrap().start.col;
        let max_col = self.vertical_segments.last().unwrap().start.col;

        Bounds { min_row, max_row, min_col, max_col }
    }
}

impl<'a> From<&'a str> for DigPlan<'a> {
    fn from(s: &'a str) -> Self {
        let instructions = s
            .lines()
            .map(|line| Instruction::from(line))
            .collect::<Vec<_>>();
        DigPlan { instructions }
    }
}

impl<'a> From<&'a str> for Instruction<'a> {
    fn from(s: &'a str) -> Self {
        let row: [&str; 3] = s.split_ascii_whitespace().collect::<Vec<_>>().try_into().unwrap();
        let direction = match row[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction")
        };
        let amount = row[1].parse::<usize>().unwrap();
        let color = &row[2][1..row[2].len() - 1];
        Instruction { direction, amount, color }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cubic_meters_of_lava() {
        let input = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

        assert_eq!(cubic_meters_of_lava(input), 62);
    }

    #[test]
    fn test_cubic_meters_of_lava_2() {
        let input = r#"L 6 (#70c710)
U 5 (#0dc571)
R 6 (#5713f0)
D 5 (#d2c081)"#;

        assert_eq!(cubic_meters_of_lava(input), 6 * 7);
    }
}
