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