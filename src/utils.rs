
#[derive(Clone, Copy)]
pub struct Coord {
    pub height: u8,
    pub width: u8
}

impl Coord {
   pub fn new(h: u8, w: u8) -> Self {
       Self {
           height: h,
           width: w
       }
   } 
}

