use std::cmp::Ordering;
use crate::geom::id_point::IdPoint;
use crate::geom::id_segment::IdSegment;
use crate::geom::index_segm::IndexSegment;
use crate::geom::point::Point;
use crate::geom::segm::Segment;

pub trait StartEnd: Clone + Send + Sync + Copy {
    fn start(&self) -> &Point;
    fn end(&self) -> &Point;

    #[inline(always)]
    fn cmp_by_start(&self, other: &Self) -> Ordering {
        let a = self.start();
        let b = other.start();
        a.x.cmp(&b.x)
    }
}

impl StartEnd for Segment {
    #[inline(always)]
    fn start(&self) -> &Point {
        &self.a
    }
    #[inline(always)]
    fn end(&self) -> &Point {
        &self.b
    }
}

impl StartEnd for IndexSegment {
    #[inline(always)]
    fn start(&self) -> &Point {
        &self.seg.a
    }
    #[inline(always)]
    fn end(&self) -> &Point {
        &self.seg.b
    }
}

impl StartEnd for IdSegment {
    #[inline(always)]
    fn start(&self) -> &Point {
        &self.a.point
    }
    #[inline(always)]
    fn end(&self) -> &Point {
        &self.b.point
    }
}

impl StartEnd for IdPoint {
    #[inline(always)]
    fn start(&self) -> &Point {
        &self.point
    }
    #[inline(always)]
    fn end(&self) -> &Point {
        &self.point
    }
}