use crate::geom::point::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Segment { // 16
    pub a: Point, // 8
    pub b: Point  // 8
}

impl Segment {
    #[inline(always)]
    pub fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }
}
