use crate::share::Pos;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Snake {
    pub body: Vec<Pos>,
    pub direction: Direction,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            body: vec![Pos { x: 2, y: 1 }, Pos { x: 3, y: 1 }, Pos { x: 4, y: 1 }],
            direction: Direction::Right,
        }
    }

    pub fn update_pos(&mut self) {
        let &Pos { x, y } = self.body.first().unwrap();
        let new_head = match self.direction {
            Direction::Up => Pos { x, y: y - 1 },
            Direction::Down => Pos { x, y: y + 1 },
            Direction::Left => Pos { x: x - 2, y },
            Direction::Right => Pos { x: x + 2, y },
        };

        self.body.insert(0, new_head);
        self.body.pop();
    }

    pub fn update_direction(&mut self, direction: Direction) {
        if self.direction == Direction::Up && direction == Direction::Down {
            return;
        } else if self.direction == Direction::Down && direction == Direction::Up {
            return;
        } else if self.direction == Direction::Left && direction == Direction::Right {
            return;
        } else if self.direction == Direction::Right && direction == Direction::Left {
            return;
        }

        self.direction = direction;
    }
}

