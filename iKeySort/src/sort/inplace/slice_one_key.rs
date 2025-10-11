use crate::sort::bin_layout::BinLayout;
use crate::sort::key::{KeyFn, SortKey};

pub(crate) trait OneKeyInPlaceSortSerial<T> {
    fn ser_sort_inplace_by_one_key<K: SortKey, F: KeyFn<T, K>>(
        &mut self,
        key: F,
    );
}

impl<T: Copy> OneKeyInPlaceSortSerial<T> for [T] {
    #[inline]
    fn ser_sort_inplace_by_one_key<K: SortKey, F: KeyFn<T, K>>(
        &mut self,
        key: F,
    ) {
        if let Some(layout) = BinLayout::with_keys(self, key) {
            layout.sort_by_one_key(self, key);
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use alloc::vec::Vec;
    use crate::sort::inplace::slice_one_key::OneKeyInPlaceSortSerial;

    #[test]
    fn test_array_0() {
        test_array(vec![2, 1]);
    }

    #[test]
    fn test_array_1() {
        test_array(vec![2, 2, 1]);
    }

    #[test]
    fn test_array_2() {
        test_array(vec![5, 2, 3, 4]);
    }

    #[test]
    fn test_array_3() {
        test_array(vec![7, 3, 2, 5, 3]);
    }

    #[test]
    fn test_array_4() {
        test_array(vec![0, 0, 0, 1, 1, 0, 0 ]);
    }

    #[test]
    fn test_0() {
        test(17);
    }

    #[test]
    fn test_1() {
        test(100);
    }

    #[test]
    fn test_2() {
        test(1_000);
    }

    #[test]
    fn test_3() {
        test(10_000);
    }

    #[test]
    fn test_4() {
        test(100_000);
    }

    #[test]
    fn test_5() {
        test(1000_000);
    }

    fn test(count: usize) {
        let mut org: Vec<_> = (0..count).rev().collect();
        let mut arr = org.clone();
        arr.ser_sort_inplace_by_one_key(|&a| a);
        org.sort_unstable();
        assert!(arr == org);
    }

    fn test_array(tmp: Vec<i32>) {
        let mut org = tmp.clone();
        let mut arr = org.clone();
        arr.ser_sort_inplace_by_one_key(|&a| a);
        org.sort_unstable();
        assert!(arr == org);
    }
}