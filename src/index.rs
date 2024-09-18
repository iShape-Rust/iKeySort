pub struct BinLayout {
    pub(super) min_key: i32,
    pub(super) power: usize
}

impl BinLayout {
    #[inline(always)]
    pub fn index(&self, value: i32) -> usize {
        (value - self.min_key) as usize >> self.power
    }
}

pub trait BinKey {
    fn key(&self) -> i32;
    fn bin(&self, layout: &BinLayout) -> usize;
}