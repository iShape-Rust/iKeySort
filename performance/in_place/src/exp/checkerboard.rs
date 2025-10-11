use crate::geom::id_segment::IdSegment;
use crate::geom::index_segm::IndexSegment;
use crate::geom::point::Point;
use crate::geom::segm::Segment;
use crate::solver::sort::SortSolution;

pub struct CheckerboardTest {
    segments: Vec<Segment>,
    index_segments: Vec<IndexSegment>,
    id_segments: Vec<IdSegment>,
}

impl CheckerboardTest {

    pub fn new(n: usize) -> Self {
        let segments = Self::checkerboard_segments(20, 30, n);
        let index_segments: Vec<_> = segments.iter().enumerate().map(|(i, s)|IndexSegment::new(i, s)).collect();
        let id_segments: Vec<_> = segments.iter().enumerate().map(|(i, s)|IdSegment::new(i, s)).collect();
        Self {
            segments,
            index_segments,
            id_segments,
        }
    }

    pub fn run_all(&self) {
        println!("Checkerboard");
        println!();
        self.run_segments();
        println!();
        self.run_index_segments();
        println!();
        self.run_id_segments();
    }

    #[allow(dead_code)]
    pub fn run_custom(&self) {
        println!("Custom test n = {}", self.segments.len());
        println!();
        SortSolution::run_segments_par_bin_sort(&self.segments);
    }

    pub fn run_segments(&self) {
        println!("Segment size:  {} bytes", size_of::<Segment>());
        println!("Segments test n = {}", self.segments.len());
        println!();
        let hash_0 = SortSolution::run_segments_sort_stable(&self.segments);
        let hash_1 = SortSolution::run_segments_sort_unstable(&self.segments);
        let hash_2 = SortSolution::run_segments_par_sort_stable(&self.segments);
        let hash_3 = SortSolution::run_segments_par_sort_unstable(&self.segments);
        let hash_4 = SortSolution::run_segments_bin_sort(&self.segments);
        let hash_5 = SortSolution::run_segments_par_bin_sort(&self.segments);
        let hash_6 = SortSolution::run_segments_inplace_sort(&self.segments);

        SortSolution::run_compare(&self.segments);
        println!("---");
    }

    pub fn run_index_segments(&self) {
        println!("Segment size:  {} bytes", size_of::<IndexSegment>());
        println!("Index Segments test n = {}", self.index_segments.len());
        println!();
        let hash_0 = SortSolution::run_segments_sort_stable(&self.index_segments);
        let hash_1 = SortSolution::run_segments_sort_unstable(&self.index_segments);
        let hash_2 = SortSolution::run_segments_par_sort_stable(&self.index_segments);
        let hash_3 = SortSolution::run_segments_par_sort_unstable(&self.index_segments);
        let hash_4 = SortSolution::run_segments_bin_sort(&self.index_segments);
        let hash_5 = SortSolution::run_segments_par_bin_sort(&self.index_segments);
        let hash_6 = SortSolution::run_segments_inplace_sort(&self.index_segments);

        println!("---");
    }

    pub fn run_id_segments(&self) {
        println!("Segment size:  {} bytes", size_of::<IdSegment>());
        println!("Id Segments test n = {}", self.id_segments.len());
        println!();
        let hash_0 = SortSolution::run_segments_sort_stable(&self.id_segments);
        let hash_1 = SortSolution::run_segments_sort_unstable(&self.id_segments);
        let hash_2 = SortSolution::run_segments_par_sort_stable(&self.id_segments);
        let hash_3 = SortSolution::run_segments_par_sort_unstable(&self.id_segments);
        let hash_4 = SortSolution::run_segments_bin_sort(&self.id_segments);
        let hash_5 = SortSolution::run_segments_par_bin_sort(&self.id_segments);
        let hash_6 = SortSolution::run_segments_inplace_sort(&self.id_segments);

        println!("---");
    }

    fn checkerboard_segments(size: i32, offset: i32, n: usize) -> Vec<Segment> {
        let mut vec = Vec::with_capacity(n * n);
        let start = Point::new(0, 0);
        let mut y = start.y;
        for _ in 0..n {
            let mut x = start.x;
            for _ in 0..n {
                let p0 = Point::new(x, y);
                let p1 = Point::new(x, y + size);
                let p2 = Point::new(x + size, y + size);
                let p3 = Point::new(x + size, y);

                let s0 = Segment::new(p0, p1);
                let s1 = Segment::new(p1, p2);
                let s2 = Segment::new(p2, p3);
                let s3 = Segment::new(p3, p0);

                vec.push(s0);
                vec.push(s1);
                vec.push(s2);
                vec.push(s3);

                x += offset;
            }
            y += offset;
        }

        vec
    }
}
