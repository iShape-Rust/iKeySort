use rand::RngExt;
use crate::geom::id_segment::IdSegment;
use crate::geom::index_segm::IndexSegment;
use crate::geom::point::Point;
use crate::geom::segm::Segment;
use crate::solver::sort::SortSolution;

pub struct RandomTest {
    segments: Vec<Segment>,
    index_segments: Vec<IndexSegment>,
    id_segments: Vec<IdSegment>,
}

impl RandomTest {
    pub fn new(n: usize) -> Self {
        let segments = Self::random_segments(n, -1_000_000_000, 1_000_000_000, 10_000);
        let index_segments: Vec<_> = segments
            .iter()
            .enumerate()
            .map(|(i, s)| IndexSegment::new(i, s))
            .collect();
        let id_segments: Vec<_> = segments
            .iter()
            .enumerate()
            .map(|(i, s)| IdSegment::new(i, s))
            .collect();
        Self {
            segments,
            index_segments,
            id_segments,
        }
    }

    pub fn run_all(&self) {
        println!("Random");
        println!();
        self.run_segments();
        println!();
        self.run_index_segments();
        println!();
        self.run_id_segments();
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

        debug_assert_eq!(hash_0, hash_1);
        debug_assert_eq!(hash_0, hash_2);
        debug_assert_eq!(hash_0, hash_3);
        debug_assert_eq!(hash_0, hash_4);
        debug_assert_eq!(hash_0, hash_5);

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

        debug_assert_eq!(hash_0, hash_1);
        debug_assert_eq!(hash_0, hash_2);
        debug_assert_eq!(hash_0, hash_3);
        debug_assert_eq!(hash_0, hash_4);
        debug_assert_eq!(hash_0, hash_5);
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

        debug_assert_eq!(hash_0, hash_1);
        debug_assert_eq!(hash_0, hash_2);
        debug_assert_eq!(hash_0, hash_3);
        debug_assert_eq!(hash_0, hash_4);
        debug_assert_eq!(hash_0, hash_5);

        println!("---");
    }

    fn random_segments(n: usize, min: i32, max: i32, len: i32) -> Vec<Segment> {
        let mut vec = Vec::with_capacity(n);
        let mut rng = rand::rng();

        for _ in 0..n {
            let x0 = rng.random_range(min..max);
            let y0 = rng.random_range(min..max);
            let dx = rng.random_range(0..len);
            let dy = rng.random_range(0..len);

            let x1 = x0 + dx;
            let y1 = y0 + dy;

            let a = Point::new(x0, y0);
            let b = Point::new(x1, y1);

            vec.push(Segment::new(a, b));
        }

        vec
    }
}
