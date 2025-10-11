use crate::sort::bin_layout::MAX_BINS_COUNT;
use core::ops::Range;
use core::slice::Iter;

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Chunk {
    start: usize,
    end: usize,
    count: usize,
}

pub struct Mapper {
    pub(crate) count: usize,
    pub(crate) chunks: [Chunk; MAX_BINS_COUNT],
}

impl Mapper {
    #[inline(always)]
    pub(crate) fn new(count: usize) -> Self {
        debug_assert!(count <= MAX_BINS_COUNT);
        Self {
            count,
            chunks: [Chunk::default(); MAX_BINS_COUNT],
        }
    }

    #[inline(always)]
    pub(super) fn inc_bin_count(&mut self, chunk_index: usize) {
        unsafe { self.chunks.get_unchecked_mut(chunk_index).count += 1 };
    }

    #[inline(always)]
    pub(super) fn next_index(&mut self, chunk_index: usize) -> usize {
        unsafe { self.chunks.get_unchecked_mut(chunk_index) }.next_index()
    }

    #[inline(always)]
    pub(super) fn init_indices(&mut self) {
        let mut offset = 0;
        for chunk in &mut self.chunks[..self.count] {
            chunk.start = offset;
            chunk.end = offset + chunk.count;
            offset += chunk.count;
            chunk.count = 0;
        }
    }

    #[inline(always)]
    pub(crate) fn iter(&self) -> Iter<'_, Chunk> {
        unsafe { self.chunks.get_unchecked(..self.count) }.iter()
    }
}

impl Chunk {
    #[inline(always)]
    pub(crate) fn as_range(&self) -> Range<usize> {
        self.start..self.end
    }

    #[inline(always)]
    pub(super) fn has_next(&self) -> bool {
        self.start + self.count < self.end
    }

    #[inline(always)]
    pub(super) fn next_index(&mut self) -> usize {
        let index = self.start + self.count;
        self.count += 1;
        index
    }
}
