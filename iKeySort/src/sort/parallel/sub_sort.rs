use crate::sort::bin_layout::MAX_BINS_COUNT;
use core::ops::Deref;
use rayon::prelude::*;

pub(super) struct BinMarks {
    values: [usize; MAX_BINS_COUNT],
    len: usize,
}

impl BinMarks {
    pub(super) fn new() -> Self {
        Self {
            values: [0; MAX_BINS_COUNT],
            len: 0,
        }
    }

    pub(super) fn push(&mut self, mark: usize) {
        self.values[self.len] = mark;
        self.len += 1;
    }
}

impl Deref for BinMarks {
    type Target = [usize];

    fn deref(&self) -> &Self::Target {
        &self.values[..self.len]
    }
}

pub(super) struct SubSortFragments<'a, T> {
    items: [Option<SubSortFragment<'a, T>>; MAX_BINS_COUNT],
    len: usize,
}

impl<'a, T> SubSortFragments<'a, T> {
    fn new() -> Self {
        Self {
            items: [const { None }; MAX_BINS_COUNT],
            len: 0,
        }
    }

    fn push(&mut self, fragment: SubSortFragment<'a, T>) {
        self.items[self.len] = Some(fragment);
        self.len += 1;
    }
}

impl<'a, T: Send> SubSortFragments<'a, T> {
    #[inline]
    pub(super) fn par_iter_mut(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut SubSortFragment<'a, T>> {
        self.items[..self.len]
            .par_iter_mut()
            .map(|fragment| fragment.as_mut().unwrap())
    }
}

pub(super) struct SubSortFragment<'a, T> {
    pub(super) src: &'a mut [T],
    pub(super) buf: &'a mut [T],
}

pub(super) trait FragmentationByMarks<T> {
    fn fragment_by_marks<'a>(
        &'a mut self,
        buf: &'a mut [T],
        marks: &[usize],
    ) -> SubSortFragments<'a, T>;
}

impl<T> FragmentationByMarks<T> for [T] {
    #[inline]
    fn fragment_by_marks<'a>(
        &'a mut self,
        buffer: &'a mut [T],
        marks: &[usize],
    ) -> SubSortFragments<'a, T> {
        debug_assert_eq!(self.len(), buffer.len());

        let mut frags = SubSortFragments::new();

        let mut src = self;
        let mut buf = buffer;

        let mut base = 0;
        for &m in marks.iter() {
            debug_assert!(m >= base, "marks must be non-decreasing");
            debug_assert!(m <= base + src.len(), "mark {m} out of bounds");

            let md = m - base;
            let (left_src, right_src) = src.split_at_mut(md);
            let (left_buf, right_buf) = buf.split_at_mut(md);

            frags.push(SubSortFragment {
                src: left_src,
                buf: left_buf,
            });

            src = right_src;
            buf = right_buf;

            base = m;
        }

        if !src.is_empty() {
            frags.push(SubSortFragment { src, buf });
        }

        frags
    }
}
