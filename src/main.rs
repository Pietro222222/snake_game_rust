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

fn get_apples(amount: u8) -> Vec<Apple> {
    let mut vec: Vec<Apple> = vec![];
    for i in 0..amount {
        vec.push(Apple::new(Apple::get_random_coord()));
    }
    vec
}

fn draw_array_in_grid<T: GridDrawable>(arr: &Vec<T>, grid: &mut Grid) {
    for i in arr {
        i.draw_in_grid(grid);
    }
}

fn did_snake_eat_any<'a>(
    apples: &'a mut Vec<Apple>,
    snake: &Snake,
) -> (bool, Option<&'a mut Apple>) {
    for i in apples {
        if snake.snake_in_apple(i) {
            return (true, Some(i));
        }
    }
    (false, None)
}

fn main() {
    let mut window = init_game();
    window.keypad(true);

    let mut grid = Grid::new(GRID_HEIGHT, GRID_WIDTH);
    let mut snake = Snake::new();
    let mut timer_count = 0;
    let mut apples = get_apples(40);
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
            let res = did_snake_eat_any(&mut apples, &snake);
            if res.0 == true {
                let mut apple = res.1.unwrap();
                apple.set_coord(Apple::get_random_coord());
                snake.add_new_piece();
            }
            // if snake.snake_in_apple(&apple) {
            //     snake.add_new_piece();
            //     apple.set_coord(Apple::get_random_coord());
            // }
            timer_count = 0;
        } else {
            timer_count += 1;
        }
        grid.init();
        //apple.draw_in_grid(&mut grid);
        draw_array_in_grid(&apples, &mut grid);
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
