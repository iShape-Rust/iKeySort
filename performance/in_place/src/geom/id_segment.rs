use crate::geom::id_point::IdPoint;
use crate::geom::segm::Segment;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct IdSegment { // 40
    pub a: IdPoint, // 16
    pub b: IdPoint, // 16
    pub index: usize // 8
}

impl IdSegment {
    #[inline(always)]
    pub fn new(index: usize, s: &Segment) -> Self {
        Self { index, a: IdPoint::new(index, s.a), b: IdPoint::new(index, s.b) }
    }
}