use pancurses::{has_colors, init_pair, start_color, Window, COLOR_PAIR};

pub struct Grid {
    height: u8,
    width: u8,
    grid: Vec<Vec<char>>,
    color: bool,
}

impl Grid {
    pub fn new(h: u8, w: u8) -> Self {
        let grid: Vec<Vec<char>> = vec![vec!['#'; w as usize]; h as usize];
        let color = has_colors();
        if color {
            start_color();
        }

        init_pair(1, 7, 2);
        init_pair(2, 7, 1);
        init_pair(3, 7, 0);

        Self {
            height: h,
            width: w,
            grid: grid,
            color: color,
        }
    }

    pub fn init(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                let character = self
                    .grid
                    .get_mut(i as usize)
                    .unwrap()
                    .get_mut(j as usize)
                    .unwrap();
                *character = '#';
            }
        }
    }

    pub fn draw_in_pos(&mut self, y: u8, x: u8, c: char) {
        let character = self
            .grid
            .get_mut(y as usize)
            .unwrap()
            .get_mut(x as usize)
            .unwrap();
        *character = c;
    }

    pub fn draw(&self, window: &mut Window) {
        for i in 0..self.height {
            for j in 0..self.width {
                let character = self.grid.get(i as usize).unwrap().get(j as usize).unwrap();
                window.mv(i.into(), j.into());
                if self.color {
                    if *character == '=' || *character == '@' {
                        window.attrset(COLOR_PAIR(1));
                        window.addch(character.to_owned());
                        window.attroff(COLOR_PAIR(1));
                    } else if *character == 'O' {
                        window.attrset(COLOR_PAIR(2));
                        window.addch(character.to_owned());
                        window.attroff(COLOR_PAIR(2));
                    } else {
                        window.attrset(COLOR_PAIR(3));
                        window.addch(character.to_owned());
                        window.attroff(COLOR_PAIR(3));
                    }
                } else {
                    window.addch(character.to_owned());
                }
            }
        }
        window.refresh();
    }
}

pub trait GridDrawable {
    fn draw_in_grid(&self, grid: &mut Grid);
}
