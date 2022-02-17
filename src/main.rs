mod apple;
mod constants;
mod game;
mod grid;
mod snake;
mod utils;

use apple::Apple;
use constants::*;
use game::*;
use grid::*;
use pancurses::{endwin, napms, Input};
use snake::*;

fn main() {
    let mut window = init_game();
    window.keypad(true);

    let mut grid = Grid::new(GRID_HEIGHT, GRID_WIDTH);
    let mut snake = Snake::new();
    let mut timer_count = 0;
    let mut apple = Apple::new(Apple::get_random_coord());
    snake.draw_in_grid(&mut grid);

    grid.draw(&mut window);

    'mainloop: loop {
        if let Some(ch) = window.getch() {
            match ch {
                Input::KeyExit => {
                    break 'mainloop;
                }
                Input::KeyUp => {
                    snake.direction = SnakeDirection::Up;
                }
                Input::KeyDown => {
                    snake.direction = SnakeDirection::Down;
                }
                Input::KeyLeft => {
                    snake.direction = SnakeDirection::Left;
                }
                Input::KeyRight => {
                    snake.direction = SnakeDirection::Right;
                }
                Input::Character(c) => {
                    if c == 'q' || c == 'Q' {
                        break 'mainloop;
                    } else if c == 'w' || c == 'W' {
                        snake.direction = SnakeDirection::Up;
                    } else if c == 'a' || c == 'A' {
                        snake.direction = SnakeDirection::Left;
                    } else if c == 's' || c == 'S' {
                        snake.direction = SnakeDirection::Down;
                    } else if c == 'd' || c == 'D' {
                        snake.direction = SnakeDirection::Right;
                    }
                }
                _ => {}
            }
        }
        if (timer_count >= 4) {
            snake.move_snake();
            if snake.is_game_over() {
                break 'mainloop;
            }
            if snake.snake_in_apple(&apple) {
                snake.add_new_piece();
                apple.set_coord(Apple::get_random_coord());
            }
            timer_count = 0;
        } else {
            timer_count += 1;
        }
        grid.init();
        apple.draw_in_grid(&mut grid);
        snake.draw_in_grid(&mut grid);
        grid.draw(&mut window);
        window.refresh();
        napms(20);
    }

    for i in 0..GRID_HEIGHT {
        for j in 0..GRID_WIDTH {
            window.mvprintw(i as i32, j as i32, " ");
        }
    }

    window.mvprintw(10, 10, "GAME OVER! PRESS ANYTHING TO QUIT");
    window.refresh();

    window.nodelay(false);
    window.getch();

    endwin();
}
