use crate::geom::id_segment::IdSegment;
use crate::geom::index_segm::IndexSegment;
use crate::geom::point::Point;
use crate::geom::segm::Segment;
use crate::solver::sort::SortSolution;

pub struct SpiralTest {
    segments: Vec<Segment>,
    index_segments: Vec<IndexSegment>,
    id_segments: Vec<IdSegment>,
}

impl SpiralTest {
    pub fn new(n: usize) -> Self {
        let segments = Self::spiral_segments(10, n);
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
        println!("Spiral");
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

    fn spiral_segments(s: i32, count: usize) -> Vec<Segment> {
        let contour = Self::spiral_contour(s, count);
        let mut result = Vec::with_capacity(contour.len());
        let mut p0 = *contour.last().unwrap();
        for &pi in contour.iter() {
            result.push(Segment::new(p0, pi));
            p0 = pi;
        }

        result
    }

    fn spiral_contour(s: i32, count: usize) -> Vec<Point> {
        let mut path_0: Vec<Point> = Vec::with_capacity(count);
        let mut path_1: Vec<Point> = Vec::with_capacity(count / 2);

        let mut s0 = s;
        let mut s1 = 2 * s;

        let mut x0 = 0;
        let mut y0 = 0;

        let mut x1 = 0;
        let mut y1 = 0;

        y0 += s0;
        path_0.push(Point::new(x0, y0));

        x0 += s0;
        path_0.push(Point::new(x0, y0));

        path_1.push(Point::new(x1, y1));

        x1 += s1;
        path_1.push(Point::new(x1, y1));
        s1 += s;

        let n = count - 4;

        for i in 0..n / 2 {
            match i % 4 {
                0 => {
                    y0 += s0;
                    y1 += s1;
                }
                1 => {
                    x0 -= s0;
                    x1 -= s1;
                }
                2 => {
                    y0 -= s0;
                    y1 -= s1;
                }
                _ => {
                    x0 += s0;
                    x1 += s1;
                }
            }
            path_0.push(Point::new(x0, y0));
            path_1.push(Point::new(x1, y1));

            s0 += s;
            s1 += s;
        }

        path_1.extend(path_0.into_iter().rev());
        path_1
    }
}
