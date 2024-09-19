use std::cmp::Ordering;
use crate::index::{BinKey, BinLayout};

#[derive(Debug, Clone)]
pub struct Bin {
    pub offset: usize,
    pub data: usize,
}

pub trait KeyBinSort {
    type Item;
    fn sort_by_bins(&mut self) -> Vec<Bin>;
    fn sort_with_bins<F>(&mut self, compare: F)
    where
        F: Fn(&Self::Item, &Self::Item) -> Ordering;
    fn sort_unstable_with_bins<F>(&mut self, compare: F)
    where
        F: Fn(&Self::Item, &Self::Item) -> Ordering;
}

impl<T: BinKey + Clone> KeyBinSort for [T] {
    type Item = T;

    fn sort_by_bins(&mut self) -> Vec<Bin> {
        let mut min_key = if let Some(item) = self.first() {
            item.key()
        } else {
            return vec![];
        };

        let mut max_key = min_key;

        for p in self.iter() {
            let key = p.key();
            min_key = key.min(min_key);
            max_key = key.max(max_key);
        }

        let delta = (max_key - min_key) as usize;
        let max_possible_bin_count = delta.min(self.len() >> 1).min(8192);
        if max_possible_bin_count <= 1 {
            return vec![Bin { offset: 0, data: self.len() }];
        }

        let scale = delta / max_possible_bin_count;
        let scale_power = log2(scale);
        let layout = BinLayout { min_key, power: scale_power };

        let bin_count = layout.index(max_key) + 1;

        let mut bins = vec![Bin { offset: 0, data: 0 }; bin_count];

        for p in self.iter() {
            let index = p.bin(&layout);
            unsafe { bins.get_unchecked_mut(index) }.data += 1;
        }

        let mut offset = 0;
        for bin in bins.iter_mut() {
            let next_offset = offset + bin.data;
            bin.offset = offset;
            bin.data = offset;
            offset = next_offset;
        }

        let copy = self.to_vec();

        for p in copy.into_iter() {
            let index = p.bin(&layout);
            let bin = unsafe { bins.get_unchecked_mut(index) };
            *unsafe { self.get_unchecked_mut(bin.data) } = p;
            bin.data += 1;
        }

        bins
    }

    fn sort_with_bins<F>(&mut self, compare: F)
    where
        F: Fn(&T, &T) -> Ordering,
    {
        if self.len() <= 16 {
            self.sort_by(|a, b| compare(a, b));
            return;
        }
        let bins = self.sort_by_bins();

        for bin in bins.iter() {
            let start = bin.offset;
            let end = bin.data;
            if start < end {
                self[start..end].sort_by(|a, b| compare(a, b));
            }
        }
    }

    fn sort_unstable_with_bins<F>(&mut self, compare: F)
    where
        F: Fn(&T, &T) -> Ordering,
    {
        if self.len() <= 16 {
            self.sort_unstable_by(|a, b| compare(a, b));
            return;
        }

        let bins = self.sort_by_bins();

        for bin in bins.iter() {
            let start = bin.offset;
            let end = bin.data;
            if start < end {
                self[start..end].sort_unstable_by(|a, b| compare(a, b));
            }
        }
    }
}

fn log2(value: usize) -> usize {
    let n = value.leading_zeros();
    (usize::BITS - n) as usize
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::ops::Range;
    use rand::Rng;
    use crate::index::{BinKey, BinLayout};
    use crate::key_sort::KeyBinSort;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        fn new(x: i32, y: i32) -> Self {
            Point { x, y }
        }
    }

    impl PartialOrd for Point {
        #[inline(always)]
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Point {
        #[inline(always)]
        fn cmp(&self, other: &Self) -> Ordering {
            let x = self.x == other.x;
            if x && self.y == other.y {
                Ordering::Equal
            } else if self.x < other.x || x && self.y < other.y {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    }

    impl BinKey for Point {
        #[inline(always)]
        fn key(&self) -> i64 {
            self.x as i64
        }

        #[inline(always)]
        fn bin(&self, layout: &BinLayout) -> usize {
            layout.index(self.key())
        }
    }

    #[test]
    fn test_0() {
        let points = vec![
            Point::new(13, 2),
            Point::new(1, 2),
            Point::new(10, 2),
            Point::new(4, 0),
            Point::new(8, 0),
            Point::new(7, 4),
            Point::new(8, 2),
            Point::new(10, 2),
            Point::new(14, 2),
        ];

        let mut ordered = points.clone();
        ordered.sort_unstable_by(|a, b| a.cmp(b));

        let mut result = points.clone();
        let bins = result.sort_by_bins();

        // Sort each bin using the provided comparison function
        for bin in bins.iter() {
            let start = bin.offset;
            let end = bin.data;
            if start < end {
                result[start..end].sort_unstable_by(|a, b| a.cmp(b));
            }
        }

        assert_eq!(ordered, result);
    }

    #[test]
    fn test_random() {
        for i in 0..1000 {
            let points = random_points(i, -1000..1000, -1000..1000);
            let mut arr_0 = points.clone();
            arr_0.sort_by(|a, b| a.cmp(b));

            let mut arr_1 = points.clone();
            arr_1.sort_with_bins(|a, b| a.cmp(b));

            let mut arr_2 = points.clone();
            arr_2.sort_unstable_with_bins(|a, b| a.cmp(b));

            assert_eq!(arr_0, arr_1);
        }
    }

    fn random_points(n: usize, x_range: Range<i32>, y_range: Range<i32>) -> Vec<Point> {
        let mut points = Vec::with_capacity(n);
        let mut rng = rand::thread_rng();
        for _ in 0..n {
            let x = rng.gen_range(x_range.clone());
            let y = rng.gen_range(y_range.clone());
            points.push(Point { x, y })
        }

        points
    }
}