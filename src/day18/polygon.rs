use crate::day18::lavaduct_lagoon::{DigPlan, Direction, Instruction};

#[derive(Clone, Debug)]
pub(crate) struct Vertex {
    pub(crate) row: i64,
    pub(crate) col: i64,
}

#[derive(Debug)]
pub(crate) struct Segment {
    pub(crate) start: Vertex,
    pub(crate) end: Vertex,
    pub(crate) kind: SegmentKind,
}

#[derive(Debug)]
pub(crate) enum SegmentKind {
    Horizontal,
    Vertical,
}

pub(crate) struct Polygon {
    pub(crate) segments: Vec<Segment>,
}

impl Polygon {
    pub(crate) fn from_gid_plan(dig_plan: DigPlan) -> Self {
        let mut segments = Vec::new();
        let mut last_vertex = Vertex::new(0, 0);
        for Instruction { direction, amount, .. } in dig_plan.instructions {
            let amount = amount as i64;
            let mut segment = match direction {
                Direction::Up => Segment::new(Vertex::new(last_vertex.row - amount, last_vertex.col), last_vertex.clone(), SegmentKind::Vertical),
                Direction::Down => Segment::new(last_vertex.clone(), Vertex::new(last_vertex.row + amount, last_vertex.col), SegmentKind::Vertical),
                Direction::Left => Segment::new(Vertex::new(last_vertex.row, last_vertex.col - amount), last_vertex.clone(), SegmentKind::Horizontal),
                Direction::Right => Segment::new(last_vertex.clone(), Vertex::new(last_vertex.row, last_vertex.col + amount), SegmentKind::Horizontal),
            };
            last_vertex = match direction {
                Direction::Up | Direction::Left => segment.start.clone(),
                Direction::Down | Direction::Right => segment.end.clone(),
            };
            segments.push(segment);
        }

        Polygon { segments }
    }
}

impl Vertex {
    fn new(row: i64, col: i64) -> Self {
        Vertex { row, col }
    }
}

impl Segment {
    fn new(start: Vertex, end: Vertex, kind: SegmentKind) -> Self {
        Segment { start, end, kind }
    }
}