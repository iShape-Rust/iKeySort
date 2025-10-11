#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[inline(always)]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
