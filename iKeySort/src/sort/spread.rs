use crate::sort::bin_layout::BinLayout;
use crate::sort::key::{KeyFn, SortKey};
use crate::sort::mapper::Mapper;
use alloc::vec::Vec;
use core::mem::MaybeUninit;
use core::mem::swap;

impl<K: SortKey> BinLayout<K> {
    #[inline(always)]
    pub(crate) fn spread_with_uninit_buffer<T: Copy, F: KeyFn<T, K>>(
        &self,
        src: &mut [T],
        buf: &mut Vec<T>,
        key: F,
    ) -> Mapper {
        buf.clear();

        let need = src.len();

        if buf.capacity() < need {
            buf.reserve(need);
        }

        let scratch: &mut [MaybeUninit<T>] = &mut buf.spare_capacity_mut()[..need];

        let mut mapper = Mapper::new(self.count());
        for a in src.iter() {
            mapper.inc_bin_count(self.index(key(a)));
        }
        mapper.init_indices();

        for val in src.iter() {
            let index = mapper.next_index(self.index(key(val)));
            unsafe {
                scratch.get_unchecked_mut(index).write(*val);
            }
        }

        #[allow(clippy::uninit_vec)]
        unsafe {
            buf.set_len(need);
        }

        mapper
    }

    #[inline(always)]
    pub(crate) fn spread_with_buffer<T: Copy, F: KeyFn<T, K>>(
        &self,
        src: &mut [T],
        buf: &mut [T],
        key: F,
    ) -> Mapper {
        let mut mapper = Mapper::new(self.count());
        for a in src.iter() {
            mapper.inc_bin_count(self.index(key(a)));
        }

        mapper.init_indices();

        for val in src.iter() {
            let index = mapper.next_index(self.index(key(val)));
            unsafe {
                *buf.get_unchecked_mut(index) = *val;
            }
        }

        mapper
    }

    #[inline(always)]
    pub(crate) fn spread_inplace<T: Copy, F: KeyFn<T, K>>(&self, src: &mut [T], key: F) -> Mapper {
        let mut mapper = Mapper::new(self.count());
        for a in src.iter() {
            mapper.inc_bin_count(self.index(key(a)));
        }

        mapper.init_indices();
        debug_assert!(mapper.count >= 2);

        // last chunk must always be sorted
        for start_chunk_index in 0..mapper.count - 1 {
            let mut chunk = unsafe { mapper.chunks.get_unchecked_mut(start_chunk_index) };
            while chunk.has_next() {
                let start_index = chunk.next_index();

                let start_val = unsafe { src.get_unchecked(start_index) };
                let mut val_chunk_index = self.index(key(start_val));
                if val_chunk_index == start_chunk_index {
                    continue;
                }

                let mut val = *start_val;
                let mut target_chunk_index = val_chunk_index;

                while start_chunk_index != target_chunk_index {
                    chunk = unsafe { mapper.chunks.get_unchecked_mut(target_chunk_index) };

                    let mut target_index = chunk.next_index();
                    let mut target_val = unsafe { src.get_unchecked_mut(target_index) };
                    target_chunk_index = self.index(key(target_val));

                    while target_chunk_index == val_chunk_index {
                        target_index = chunk.next_index();
                        target_val = unsafe { src.get_unchecked_mut(target_index) };
                        target_chunk_index = self.index(key(target_val));
                    }

                    swap(target_val, &mut val);
                    val_chunk_index = target_chunk_index;
                }

                unsafe {
                    *src.get_unchecked_mut(start_index) = val;
                    chunk = mapper.chunks.get_unchecked_mut(start_chunk_index);
                };
            }
        }

        mapper
    }
}
