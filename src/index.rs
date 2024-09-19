pub struct BinLayout {
    pub(super) min_key: usize,
    pub(super) power: usize
}

impl BinLayout {
    #[inline(always)]
    pub fn index(&self, value: usize) -> usize {
        value - self.min_key >> self.power
    }
}

pub trait BinKey {
    fn key(&self) -> usize;
    fn bin(&self, layout: &BinLayout) -> usize;
}