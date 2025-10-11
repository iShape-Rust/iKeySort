use crate::geom::point::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct IdPoint {
    pub id: usize,
    pub point: Point,
}

impl IdPoint {
    #[inline(always)]
    pub fn new(id: usize, point: Point) -> Self {
        Self { id, point }
    }
}