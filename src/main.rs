/* 
   Simple Rust snake game project. 
   This is for the purposes of education to demonstrate to my student how 
   the rust programing languge works for system and game development.

   Copyright (C) 2025 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.

    Written by Frost Edson.
*/

mod objects;
mod utils;

use crate::objects::{fruit::Fruit, snake::Snake};
use k_board::{keyboard::Keyboard, keys::Keys};
use rand::Rng;
use std::process::Command;
use utils::{direction::Direction, point::Point};

#[derive(Debug)]
struct Game<'a> {
    is_game_over: bool,
    score: i32,
    fruit: &'a mut Fruit,
    snake: &'a mut Snake,
    window_width: i32,
    window_height: i32,
}

impl<'a> Game<'a> {
    fn new(fruit: &'a mut Fruit, snake: &'a mut Snake) -> Self {
        Game {
            is_game_over: false,
            score: 0,
            fruit,
            snake,
            window_width: 40,
            window_height: 20,
        }
    }
    fn setup(&mut self) {
        self.is_game_over = false;
        self.score = 0;
        self.fruit.update_position(self.generate_random_point());
    }

    fn update(&mut self) {
        self.clear_screen();
        self.snake.update(self.window_width, self.window_height);
        if self.snake.tail[0].x == self.fruit.position.x
            && self.snake.tail[0].y == self.fruit.position.y
        {
            self.increase_score();
            self.snake.grow();
            self.fruit.update_position(self.generate_random_point());
        }
    }

    fn clear_screen(&mut self) {
        if cfg!(target_os = "windows") {
            Command::new("cls").status().unwrap();
        } else {
            Command::new("clear").status().unwrap();
        }
    }

    fn draw(&mut self) {
        println!("Game Status: \nscore : {},", self.score);

        for i in 0..self.window_height {
            for j in 0..self.window_width {
                if i == 0 || i == self.window_height - 1 {
                    print!("∎");
                } else if j == 0 || j == self.window_width - 1 {
                    print!("∎");
                } else {
                    let mut drawn = false;

                    // Check for fruit first
                    if j == self.fruit.position.x && i == self.fruit.position.y {
                        self.fruit.draw();
                        drawn = true;
                    }

                    // Check for snake segments (including head)
                    if !drawn {
                        if let Some(ch) = self.snake.draw(j, i) {
                            print!("{ch}");
                            drawn = true;
                        }
                    }

                    // If nothing was drawn, print empty space
                    if !drawn {
                        print!(" ");
                    }
                }
            }
            println!();
        }
        println!();
    }

    fn process_input(&mut self) {
        for key in Keyboard::new() {
            match key {
                Keys::Char('q') => break,
                Keys::Up => {
                    if self.snake.direction != Direction::Down {
                        self.snake.direction = Direction::Up;
                    }

                    break;
                }
                Keys::Down => {
                    if self.snake.direction != Direction::Up {
                        self.snake.direction = Direction::Down;
                    }

                    break;
                }
                Keys::Left => {
                    if self.snake.direction != Direction::Right {
                        self.snake.direction = Direction::Left;
                    }

                    break;
                }
                Keys::Right => {
                    if self.snake.direction != Direction::Left {
                        self.snake.direction = Direction::Right;
                    }

                    break;
                }
                _ => break,
            }
        }
    }

    fn increase_score(&mut self) {
        self.score += 1;
    }

    fn generate_random_point(&self) -> Point {
        let mut rng = rand::rng();

        let x = rng.random_range(1..self.window_width - 1);
        let y = rng.random_range(1..self.window_height - 1);

        Point { x, y }
    }
}

fn main() {
    let mut snake = Snake::new(Point { x: 5, y: 5 }, 3);
    let mut fruit = Fruit::new(Point { x: 10, y: 10 });

    let mut game = Game::new(&mut fruit, &mut snake);
    game.setup();

    while !game.is_game_over {
        game.process_input();
        game.update();
        game.draw();
    }
}
