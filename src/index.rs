pub struct BinLayout {
    pub(super) min_key: i64,
    pub(super) power: usize
}

impl BinLayout {
    #[inline(always)]
    pub fn index(&self, value: i64) -> usize {
        ((value - self.min_key) as usize) >> self.power
    }
}

pub trait BinKey {
    fn key(&self) -> i64;
    fn bin(&self, layout: &BinLayout) -> usize;
}