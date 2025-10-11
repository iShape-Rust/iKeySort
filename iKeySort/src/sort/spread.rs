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

        // 1) Count desired occupancy per bin
        for a in src.iter() {
            mapper.inc_bin_count(self.index(key(a)));
        }
        mapper.init_indices();

        // With <=1 bin there is nothing to do.
        debug_assert!(mapper.count >= 1);
        if mapper.count <= 1 {
            return mapper;
        }

        // 2) Cycle-leader placement for all bins except the last (acts as sink)
        for home_bin in 0..mapper.count - 1 {
            // Advance within the home bin until all its target slots are filled.
            let mut home = unsafe { mapper.chunks.get_unchecked_mut(home_bin) };

            while home.has_next() {
                let write_pos = home.next_index();

                // Fast path: element already belongs to its home bin -> nothing to move.
                let k = key(unsafe { src.get_unchecked(write_pos) });
                let owner_bin = self.index(k);
                if owner_bin == home_bin {
                    continue;
                }

                // Start a cycle with the displaced value `val`.
                let mut val = unsafe { *src.get_unchecked(write_pos) };
                let mut scan_bin = owner_bin;

                // Move `val` forward until it reaches its home bin.
                while scan_bin != home_bin {
                    // We will write into the next open slot of `scan_bin`.
                    let victim_bin = unsafe { mapper.chunks.get_unchecked_mut(scan_bin) };

                    // Find a victim in `scan_bin` that does NOT belong to `scan_bin`.
                    // (By invariant, such a victim must exist before `scan_bin` is fully settled.)
                    debug_assert!(victim_bin.has_next(), "no free slot left in scan_bin");
                    let mut victim_pos = victim_bin.next_index();

                    // Skip correctly-placed elements in this bin.
                    loop {
                        let victim_key = key(unsafe { src.get_unchecked(victim_pos) });
                        let victim_owner = self.index(victim_key);
                        if victim_owner != scan_bin {
                            // We found a misplaced element; swap it out and continue the cycle.
                            let victim_ref = unsafe { src.get_unchecked_mut(victim_pos) };
                            swap(victim_ref, &mut val);
                            // The swapped-in `val` now *belongs* to `scan_bin`, so we proceed
                            // with the victim we just evicted.
                            scan_bin = victim_owner;
                            break;
                        }

                        debug_assert!(
                            victim_bin.has_next(),
                            "exhausted scan_bin while seeking victim"
                        );
                        victim_pos = victim_bin.next_index();
                    }
                }

                // `val` now belongs to `home_bin`; place it at `write_pos`.
                unsafe { *src.get_unchecked_mut(write_pos) = val };

                // Refresh `home` reference after potential aliasing through `mapper`.
                home = unsafe { mapper.chunks.get_unchecked_mut(home_bin) };
            }
        }

        // The last bin’s remaining slots must all be its own elements by construction.
        debug_assert!({
            let last = unsafe { mapper.chunks.get_unchecked(mapper.count - 1) };
            !last.has_next()
        });

        mapper
    }
}
