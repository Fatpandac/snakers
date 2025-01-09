use crate::share::Pos;

#[derive(Debug)]
pub struct Food {
    pub pos: Pos,
    pub alive: bool,
}

impl Food {
    pub fn new() -> Self {
        Self {
            pos: Pos { x: 10, y: 10 },
            alive: true,
        }
    }

    pub fn update_food(&mut self, window_width: u16, window_height: u16) {
        if !self.alive {
            self.pos = Pos {
                x: rand::random::<u16>() % window_width * 2,
                y: rand::random::<u16>() % window_height,
            };
            self.alive = true;
        }
    }
}

