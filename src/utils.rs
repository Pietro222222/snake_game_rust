#[derive(Clone, Copy)]
pub struct Coord {
    pub height: i8,
    pub width: i8,
}

impl Coord {
    pub fn new(h: i8, w: i8) -> Self {
        Self {
            height: h,
            width: w,
        }
    }
}
