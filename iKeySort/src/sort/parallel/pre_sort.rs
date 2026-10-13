use crate::sort::bin_layout::BinLayout;
use crate::sort::key::{KeyFn, SortKey};
use crate::sort::mapper::Mapper;
use core::ops::Range;
use std::cell::RefCell;
use std::mem::MaybeUninit;

thread_local! {
    // Only indices are retained; no element data or references survive a sort.
    static RANGES: RefCell<Vec<Range<usize>>> = const { RefCell::new(Vec::new()) };
}

pub(super) struct ScratchRanges {
    pub(super) ranges: Vec<Range<usize>>,
}

impl ScratchRanges {
    pub(super) fn new(len: usize) -> Self {
        // Take ownership before calling user code or entering Rayon, so nested
        // sorts can rent their own storage without holding a RefCell borrow.
        let ranges = RANGES
            .try_with(|cache| core::mem::take(&mut *cache.borrow_mut()))
            .unwrap_or_default();
        let mut scratch = Self { ranges };
        scratch.ranges.resize(len, 0..0);
        scratch
    }
}

impl Drop for ScratchRanges {
    fn drop(&mut self) {
        // A nested sort may already have returned a buffer. Keep the larger one.
        // try_with also permits sorting during thread-local destruction.
        let _ = RANGES.try_with(|cache| {
            if let Ok(mut cached) = cache.try_borrow_mut() {
                if self.ranges.capacity() > cached.capacity() {
                    core::mem::swap(&mut *cached, &mut self.ranges);
                }
            }
        });
    }
}

#[inline]
pub(super) fn spread_fragment<T, K, F>(
    src: &[T],
    buf: &mut [MaybeUninit<T>],
    ranges: &mut [Range<usize>],
    base: usize,
    layout: &BinLayout<K>,
    key: F,
) where
    T: Copy,
    K: SortKey,
    F: KeyFn<T, K>,
{
    debug_assert_eq!(src.len(), buf.len());
    debug_assert_eq!(ranges.len(), layout.count());
    let mut mapper = Mapper::new(layout.count());
    for a in src {
        mapper.inc_bin_count(layout.index(key(a)));
    }
    mapper.init_indices();

    for val in src {
        let index = mapper.next_index(layout.index(key(val)));
        // SAFETY: the histogram reserves one destination for every source item.
        unsafe { buf.get_unchecked_mut(index).write(*val) };
    }

    for (dst, chunk) in ranges.iter_mut().zip(mapper.iter()) {
        let range = chunk.as_range();
        *dst = range.start + base..range.end + base;
    }
}

#[cfg(test)]
mod tests {
    use super::ScratchRanges;

    #[test]
    fn scratch_is_available_after_thread_cache_destruction() {
        struct RentOnDrop;
        impl Drop for RentOnDrop {
            fn drop(&mut self) {
                let mut scratch = ScratchRanges::new(8);
                scratch.ranges[0] = 2..4;
                assert_eq!(scratch.ranges[0], 2..4);
            }
        }
        thread_local! {
            static RENT_ON_DROP: RentOnDrop = const { RentOnDrop };
        }
        std::thread::spawn(|| {
            // TLS is destroyed in reverse initialization order: the index cache
            // is gone when RentOnDrop requests scratch storage.
            RENT_ON_DROP.with(|_| {});
            drop(ScratchRanges::new(8));
        })
        .join()
        .unwrap();
    }
}
