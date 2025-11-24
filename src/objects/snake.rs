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

use crate::utils::{direction::Direction, point::Point};


#[derive(Debug)]
pub struct Snake {
    pub tail: Vec<Point>,
    pub direction: Direction,
    pub pending: usize,
}

impl Snake {
    pub fn new(position: Point, initial_tail_length: usize) -> Self {
        let mut tail = Vec::new();
        for i in 0..initial_tail_length {
            tail.push(Point { x: position.x - i as i32, y: position.y });
        }

        Snake {
            tail,
            direction: Direction::None,
            pending: 0,
        }
    }

    pub fn draw(&self, x: i32, y: i32) -> Option<char> {
        self.tail.iter().enumerate().find_map(|(i, seg)| {
            if seg.x == x && seg.y == y {
                Some(if i == 0 { 'O' } else { 'o' })
            } else {
                None
            }
        })
    }
    

    pub fn update(&mut self, width: i32, height: i32) {
        if let Some(head) = self.tail.first().cloned() {
            let mut new_head = match self.direction {
                Direction::Up    => Point { x: head.x,     y: head.y - 1 },
                Direction::Down  => Point { x: head.x,     y: head.y + 1 },
                Direction::Left  => Point { x: head.x - 1, y: head.y },
                Direction::Right => Point { x: head.x + 1, y: head.y },
                Direction::None  => head,
            };
            
            // Fixed boundary wrapping - use the actual width/height
            if new_head.x < 0 { 
                new_head.x = width - 1; 
            }
            if new_head.x >= width { 
                new_head.x = 0; 
            }
            if new_head.y < 0 { 
                new_head.y = height - 1; 
            }
            if new_head.y >= height { 
                new_head.y = 0; 
            }
    
            self.tail.insert(0, new_head);
            if self.pending == 0 {
                self.tail.pop();
            } else {
                self.pending -= 1;
            }
        }
    }

    pub fn grow(&mut self) {
        self.pending += 1;
    }

}
