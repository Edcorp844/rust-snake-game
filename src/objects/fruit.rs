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

use crate::utils::point::Point;


#[derive(Debug)]
pub struct Fruit {
   pub position: Point,
}

impl Fruit {
    pub fn new(position: Point) -> Fruit {
        Fruit { position }
    }

   pub  fn update_position(&mut self, new_position: Point) {
        self.position = new_position;
    }

    pub fn draw(&self){
       // print!("Ǒ");
       print!("🍎")
    }
}