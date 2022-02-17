use pancurses::{Window, initscr, raw, noecho};




pub fn init_game() -> Window {
    let mut window = initscr();
    noecho();
    raw();
    window.nodelay(true);
    return window;
}