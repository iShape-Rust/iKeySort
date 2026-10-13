use crate::sort::bin_layout::{BinLayout, LayoutConstraints, MAX_BINS_COUNT};
use crate::sort::key::{KeyFn, SortKey};
use crate::sort::parallel::pre_sort::{ScratchRanges, spread_fragment};
use crate::sort::parallel::sub_sort::BinMarks;
use core::ops::Range;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;

impl<K: SortKey + Send + Sync> BinLayout<K> {
    #[inline]
    pub(super) fn with_cpu_count<T, F>(cpu: usize, slice: &[T], key: F) -> Option<Self>
    where
        T: Copy + Send + Sync,
        F: KeyFn<T, K> + Send + Sync,
    {
        let (min_key, max_key) = slice.par_min_max(key);

        if min_key == max_key {
            // array is flat
            return None;
        }

        let max_bins_count = cpu.saturating_mul(4).min(MAX_BINS_COUNT);

        Some(BinLayout::with_constraints(
            min_key,
            max_key,
            LayoutConstraints {
                max_split_count: max_bins_count,
            },
        ))
    }
}

trait ParMinMax<T> {
    fn par_min_max<K, F>(&self, key: F) -> (K, K)
    where
        K: Copy + Ord + Send + Sync,
        T: Copy + Send + Sync,
        F: KeyFn<T, K> + Send + Sync;
}

impl<T> ParMinMax<T> for [T] {
    #[inline(always)]
    fn par_min_max<K, F>(&self, key: F) -> (K, K)
    where
        K: Copy + Ord + Send + Sync,
        T: Copy + Send + Sync,
        F: KeyFn<T, K> + Send + Sync,
    {
        debug_assert!(!self.is_empty());
        let first_val = self.first().unwrap();
        let k0 = key(first_val);

        let (min_key, max_key) = self
            .par_iter()
            .map(|v| {
                let k = key(v);
                (k, k)
            })
            .reduce(|| (k0, k0), |a, b| (a.0.min(b.0), a.1.max(b.1)));

        (min_key, max_key)
    }
}

impl<K: SortKey + Send + Sync> BinLayout<K> {
    pub(super) fn par_pre_sort<T, F>(
        &self,
        cpu: usize,
        src: &mut [T],
        buf: &mut Vec<T>,
        key: F,
    ) -> BinMarks
    where
        T: Copy + Send + Sync,
        F: KeyFn<T, K> + Send + Sync,
    {
        buf.clear();
        if buf.capacity() < src.len() {
            buf.reserve(src.len());
        }

        let bins_count = self.count();
        let step_len = src.len().div_ceil(cpu);
        let fragments_count = src.len().div_ceil(step_len);
        let ranges_count = fragments_count
            .checked_mul(bins_count)
            .expect("too many bin ranges");
        let mut scratch = ScratchRanges::new(ranges_count);

        src.par_chunks(step_len)
            .zip(buf.spare_capacity_mut()[..src.len()].par_chunks_mut(step_len))
            .zip(scratch.ranges.par_chunks_mut(bins_count))
            .enumerate()
            .for_each(|(index, ((src, buf), ranges))| {
                spread_fragment(src, buf, ranges, index * step_len, self, key);
            });

        // SAFETY: every fragment has initialized its disjoint part of the buffer.
        // If a key function panics, execution never reaches set_len.
        unsafe {
            buf.set_len(src.len());
        }

        copy_groups(src, buf, &scratch.ranges, bins_count)
    }
}

#[inline]
fn copy_groups<T: Copy>(
    dst: &mut [T],
    src: &[T],
    ranges: &[Range<usize>],
    bins_count: usize,
) -> BinMarks {
    let mut marks = BinMarks::new();
    let mut offset = 0;
    for bin in 0..bins_count {
        let start = offset;
        for row in ranges.chunks_exact(bins_count) {
            let range = &row[bin];
            if range.is_empty() {
                continue;
            }
            let end = offset + range.len();
            dst[offset..end].copy_from_slice(&src[range.clone()]);
            offset = end;
        }
        if offset > start && offset < dst.len() {
            marks.push(offset);
        }
    }
    debug_assert_eq!(offset, dst.len());
    marks
}

#[cfg(test)]
mod scratch_tests {
    use super::BinLayout;
    use crate::sort::parallel::sub_sort::FragmentationByMarks;
    use rayon::iter::ParallelIterator;
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::sync::atomic::{AtomicBool, Ordering};

    #[test]
    fn sparse_and_full_bins_reuse_scratch_with_uneven_fragments() {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(3)
            .build()
            .unwrap();
        pool.install(|| {
            let mut buffer = Vec::new();
            for bins in [256, 2, 17, 256] {
                let mut values: Vec<i64> = (0..1_031).rev().map(|i| i % bins).collect();
                let mut expected = values.clone();
                expected.sort_unstable();
                let layout = BinLayout::with_keys(&values, |x| *x).unwrap();
                let marks = layout.par_pre_sort(3, &mut values, &mut buffer, |x| *x);
                assert_eq!(values, expected);
                assert_eq!(marks.len(), bins as usize - 1);
                values
                    .fragment_by_marks(&mut buffer, &marks)
                    .par_iter_mut()
                    .for_each(|fragment| {
                        assert!(fragment.src.iter().all(|x| *x == fragment.src[0]))
                    });
            }
            let mut values = [i64::MIN, 0, i64::MAX].repeat(347);
            let mut expected = values.clone();
            expected.sort_unstable();
            let layout = BinLayout::with_keys(&values, |x| *x).unwrap();
            let marks = layout.par_pre_sort(3, &mut values, &mut buffer, |x| *x);
            assert_eq!(values, expected);
            assert_eq!(marks.len(), 2);
        });
    }

    #[test]
    fn nested_spread_can_rent_scratch_on_the_same_thread() {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .build()
            .unwrap();
        pool.install(|| {
            let mut values: Vec<i64> = (0..64).rev().collect();
            let layout = BinLayout::with_keys(&values, |x| *x).unwrap();
            let nested = AtomicBool::new(false);
            layout.par_pre_sort(1, &mut values, &mut Vec::new(), |x| {
                if !nested.swap(true, Ordering::Relaxed) {
                    let mut inner: Vec<i64> = (0..256).rev().collect();
                    let inner_layout = BinLayout::with_keys(&inner, |x| *x).unwrap();
                    inner_layout.par_pre_sort(1, &mut inner, &mut Vec::new(), |x| *x);
                    assert!(inner.is_sorted());
                }
                *x
            });
            assert!(values.is_sorted());
            assert!(nested.load(Ordering::Relaxed));
        });
    }

    #[test]
    fn panic_during_scatter_keeps_buffer_uninitialized_and_reusable() {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(2)
            .build()
            .unwrap();
        pool.install(|| {
            let mut values: Vec<i64> = (0..128).rev().collect();
            let original = values.clone();
            let layout = BinLayout::with_keys(&values, |x| *x).unwrap();
            let seen = AtomicBool::new(false);
            let mut buffer = vec![0; values.len()];
            let result = catch_unwind(AssertUnwindSafe(|| {
                layout.par_pre_sort(2, &mut values, &mut buffer, |x| {
                    if *x == 0 && seen.swap(true, Ordering::Relaxed) {
                        panic!("key failed during scatter");
                    }
                    *x
                });
            }));
            assert!(result.is_err());
            assert!(buffer.is_empty());
            assert_eq!(values, original);
            layout.par_pre_sort(2, &mut values, &mut buffer, |x| *x);
            assert!(values.is_sorted());
            assert_eq!(buffer.len(), values.len());
        });
    }
}
