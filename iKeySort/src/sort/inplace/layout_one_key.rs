use crate::sort::bin_layout::BinLayout;
use crate::sort::key::{KeyFn, SortKey};

impl<K: SortKey> BinLayout<K> {
    #[inline]
    pub(super) fn sort_by_one_key<T: Copy, F: KeyFn<T, K>>(
        &self,
        src: &mut [T],
        key: F,
    ) {
        let mapper = self.spread_inplace(src, key);

        if !self.bin_width_is_one() {
            mapper.sort_inplace_by_one_key(src, key);
        }
    }
}