use crate::geom::start_segment::StartEnd;
use i_key_sort::sort::two_keys::TwoKeysSort;
use rayon::slice::ParallelSliceMut;
use std::time::Instant;

pub struct SortSolution;

impl SortSolution {
    pub fn run_segments_sort_unstable<S: StartEnd>(segments: &[S]) -> i32 {
        let mut data = segments.to_vec();
        let start = Instant::now();

        let n = Self::repeat_count(segments.len());
        for _ in 0..n {
            data.copy_from_slice(segments);
            data.sort_unstable_by(|s0, s1| s0.cmp_by_start(s1));
        }
        let duration = start.elapsed().as_secs_f64() / (n as f64);

        Self::print_result("sort_unstable", duration);

        data.last().unwrap().end().x
    }

    pub fn run_segments_sort_stable<S: StartEnd>(segments: &[S]) -> i32 {
        let mut data = segments.to_vec();
        let start = Instant::now();

        let n = Self::repeat_count(segments.len());
        for _ in 0..n {
            data.copy_from_slice(segments);
            data.sort_by(|s0, s1| s0.cmp_by_start(s1));
        }
        let duration = start.elapsed().as_secs_f64() / (n as f64);

        Self::print_result("sort_stable", duration);

        data.last().unwrap().end().x
    }

    pub fn run_segments_par_sort_unstable<S: StartEnd>(segments: &[S]) -> i32 {
        let mut data = segments.to_vec();
        let start = Instant::now();

        let n = Self::repeat_count(segments.len());
        for _ in 0..n {
            data.copy_from_slice(segments);
            data.par_sort_unstable_by(|s0, s1| s0.cmp_by_start(s1));
        }
        let duration = start.elapsed().as_secs_f64() / (n as f64);

        Self::print_result("par_sort_unstable", duration);

        data.last().unwrap().end().x
    }

    pub fn run_segments_par_sort_stable<S: StartEnd>(segments: &[S]) -> i32 {
        let mut data = segments.to_vec();
        let start = Instant::now();

        let n = Self::repeat_count(segments.len());
        for _ in 0..n {
            data.copy_from_slice(&segments);
            data.par_sort_by(|s0, s1| s0.cmp_by_start(s1));
        }
        let duration = start.elapsed().as_secs_f64() / (n as f64);

        Self::print_result("par_sort_stable", duration);

        data.last().unwrap().end().x
    }

    pub fn run_segments_bin_sort<S: StartEnd + Copy + Default>(segments: &[S]) -> i32 {
        let mut data = segments.to_vec();
        let start = Instant::now();

        let n = Self::repeat_count(segments.len());
        for _ in 0..n {
            data.copy_from_slice(&segments);
            data.sort_by_two_keys(false, |s| s.start().x, |s| s.start().y);
        }
        let duration = start.elapsed().as_secs_f64() / (n as f64);

        Self::print_result("bin_sort", duration);

        data.last().unwrap().end().x
    }

    pub fn run_segments_par_bin_sort<S: StartEnd + Copy + Default>(segments: &[S]) -> i32 {
        let mut data = segments.to_vec();
        let start = Instant::now();

        let n = Self::repeat_count(segments.len());
        for _ in 0..n {
            data.copy_from_slice(&segments);
            data.sort_by_two_keys(true, |s| s.start().x, |s| s.start().y);
        }
        let duration = start.elapsed().as_secs_f64() / (n as f64);

        Self::print_result("par_bin_sort", duration);

        data.last().unwrap().end().x
    }

    fn print_result(title: &str, duration: f64) {
        println!("{} - {:.6}", title, duration);
    }

    pub fn run_compare<S: StartEnd + Copy + Default>(segments: &[S]) {
        let mut data_0 = segments.to_vec();
        data_0.sort_by_two_keys(true, |s| s.start().x, |s| s.start().y);
        let mut data_1 = segments.to_vec();
        data_1.sort_by_two_keys(false, |s| s.start().x, |s| s.start().y);
        let mut data_2 = segments.to_vec();
        data_2.par_sort_unstable_by(|s0, s1| s0.cmp_by_start(s1));

        if let Some(index) = compare_by_start(&data_1, &data_2) {
            println!("not valid ser sort index: {}", index);
        }
        if let Some(index) = compare_by_start(&data_0, &data_2) {
            println!("not valid par sort index: {}", index);
        }
    }

    fn repeat_count(len: usize) -> usize {
        (100_000 / len).max(1)
    }
}

fn compare_by_start<S: StartEnd>(data_1: &[S], data_2: &[S]) -> Option<usize> {
    if data_1.len() != data_2.len() {
        return Some(usize::MAX);
    }
    for (i, (s1, s2)) in data_1.iter().zip(data_2.iter()).enumerate() {
        if s1.start() != s2.start() {
            return Some(i);
        }
    }

    None
}
