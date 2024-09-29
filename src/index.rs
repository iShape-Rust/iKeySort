use std::ops::Range;

pub trait Offset {
    fn offset(self, other: Self) -> usize;
}

impl Offset for i64 {
    #[inline(always)]
    fn offset(self, other: Self) -> usize {
        self.wrapping_sub(other) as usize
    }
}

impl Offset for i32 {
    #[inline(always)]
    fn offset(self, other: Self) -> usize {
        (self as i64).wrapping_sub(other as i64) as usize
    }
}

impl Offset for usize {
    #[inline(always)]
    fn offset(self, other: Self) -> usize {
        self.wrapping_sub(other)
    }
}

pub struct BinLayout<T> {
    pub(super) min_key: T,
    pub(super) power: usize,
}

impl<T> BinLayout<T>
where
    T: Copy + Offset + PartialOrd,
{
    #[inline(always)]
    pub fn index(&self, value: T) -> usize {
        value.offset(self.min_key) >> self.power
    }

    #[inline(always)]
    pub fn new(range: Range<T>, elements_count: usize) -> Option<BinLayout<T>> {
        let delta = range.end.offset(range.start) + 1;
        let max_possible_bin_count = delta.min(elements_count >> 1).min(8192);
        if max_possible_bin_count <= 1 {
            return None;
        }

        let scale = delta / max_possible_bin_count;
        let scale_power = log2(scale);
        Some(Self {
            min_key: range.start,
            power: scale_power,
        })
    }
}

#[inline(always)]
fn log2(value: usize) -> usize {
    let n = value.leading_zeros();
    (usize::BITS - n) as usize
}

pub trait BinKey<T> {
    fn bin_key(&self) -> T;
    fn bin_index(&self, layout: &BinLayout<T>) -> usize;
}
