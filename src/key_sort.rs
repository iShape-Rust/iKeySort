use std::cmp::Ordering;
use crate::index::{BinKey, BinLayout, Offset};

#[derive(Debug, Clone)]
pub struct Bin {
    pub offset: usize,
    pub data: usize,
}

pub trait KeyBinSort<T> {
    type Item;
    fn sort_by_bins(&mut self) -> Vec<Bin>;
    fn sort_with_bins<F>(&mut self, compare: F)
    where
        F: Fn(&Self::Item, &Self::Item) -> Ordering;
    fn sort_unstable_with_bins<F>(&mut self, compare: F)
    where
        F: Fn(&Self::Item, &Self::Item) -> Ordering;
}

impl<T, U> KeyBinSort<U> for [T]
where
    T: BinKey<U> + Clone,
    U: Copy + Ord + Offset,
{
    type Item = T;

    fn sort_by_bins(&mut self) -> Vec<Bin> {
        let mut min_key = if let Some(item) = self.first() {
            item.bin_key()
        } else {
            return vec![];
        };

        let mut max_key = min_key;

        for p in self.iter() {
            let key = p.bin_key();
            min_key = key.min(min_key);
            max_key = key.max(max_key);
        }

        let layout = if let Some(layout) = BinLayout::new(min_key..max_key, self.len()) {
            layout
        } else {
            return vec![Bin { offset: 0, data: self.len() }];
        };

        let bin_count = layout.index(max_key) + 1;
        let mut bins = vec![Bin { offset: 0, data: 0 }; bin_count];

        for p in self.iter() {
            let index = p.bin_index(&layout);
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

        for item in copy.into_iter() {
            let index = item.bin_index(&layout);
            let bin = unsafe { bins.get_unchecked_mut(index) };
            *unsafe { self.get_unchecked_mut(bin.data) } = item;
            bin.data += 1;
        }

        bins
    }

    #[inline]
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

    #[inline]
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
