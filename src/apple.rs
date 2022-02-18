use crate::constants::*;
use crate::utils::Coord;
use crate::{Grid, GridDrawable};
use rand::Rng;

pub struct Apple {
    pub coord: Coord,
}

impl Apple {
    pub fn get_random_coord() -> Coord {
        let h = rand::thread_rng().gen_range(0..GRID_HEIGHT) as i8;
        let w = rand::thread_rng().gen_range(0..GRID_WIDTH) as i8;
        Coord::new(h, w)
    }
    pub fn new(coord: Coord) -> Self {
        Self { coord }
    }
    pub fn set_coord(&mut self, c: Coord) {
        self.coord = c;
    }
}

impl GridDrawable for Apple {
    fn draw_in_grid(&self, grid: &mut Grid) {
        grid.draw_in_pos(self.coord.height, self.coord.width, 'O');
    }
}
