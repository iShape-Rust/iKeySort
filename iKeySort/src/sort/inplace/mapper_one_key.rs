use crate::sort::bin_layout::BIN_SORT_MIN;
use crate::sort::inplace::slice_one_key::OneKeyInPlaceSortSerial;
use crate::sort::key::{KeyFn, SortKey};
use crate::sort::mapper::Mapper;

impl Mapper {
    #[inline]
    pub(crate) fn sort_inplace_by_one_key<K: SortKey, T: Copy, F: KeyFn<T, K>>(
        &self,
        src: &mut [T],
        key: F,
    ) {
        const TINY_SORT_MAX: usize = BIN_SORT_MIN;

        for chunk in self.iter() {
            let range = chunk.as_range();
            match range.len() {
                0..2 => continue,
                2..TINY_SORT_MAX => {
                    let sub_buf = unsafe { src.get_unchecked_mut(range.clone()) };
                    sub_buf.sort_unstable_by_key(key);
                }
                _ => {
                    let sub_buf = unsafe { src.get_unchecked_mut(range.clone()) };
                    sub_buf.ser_sort_inplace_by_one_key(key);
                }
            }
        }
    }
}