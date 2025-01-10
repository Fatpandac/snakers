use crate::food::Food;
use crate::fps::Fps;
use crate::share::{Pos, BLOCK};
use crate::snake::{Direction, Snake};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    layout::{Rect, Size},
    prelude::CrosstermBackend,
    widgets::{Block, Borders},
    Frame, Terminal,
};
use std::{io::Stdout, time::Duration};

#[derive(Debug)]
pub struct Game {
    window_width: u16,
    window_height: u16,
    snake: Snake,
    food: Food,
    fps: Fps,
    pub is_over: bool,
}

impl Game {
    pub fn new(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Game {
        let Size { width, height } = terminal.size().unwrap();

        Self {
            window_width: width,
            window_height: height,
            snake: Snake::new(),
            food: Food::new(),
            fps: Fps::new(20),
            is_over: false,
        }
    }

    fn get_input(&self) -> Option<KeyCode> {
        fn free_key_event() {
            loop {
                if let Ok(true) = event::poll(Duration::from_millis(0)) {
                    event::read().expect("Failed to read event");
                    continue;
                }

                break;
            }
        }

        if let Ok(true) = event::poll(Duration::from_millis(0)) {
            if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
                free_key_event();
                return Some(code);
            }
        }

        return None;
    }

    fn opration(&mut self) {
        if let Some(key_code) = self.get_input() {
            return match key_code {
                KeyCode::Char('q') => {
                    self.is_over = true;
                },
                KeyCode::Char('w') | KeyCode::Char('k') | KeyCode::Up => {
                    self.snake.update_direction(Direction::Up);
                }
                KeyCode::Char('s') | KeyCode::Char('j') | KeyCode::Down => {
                    self.snake.update_direction(Direction::Down);
                }
                KeyCode::Char('a') | KeyCode::Char('h') | KeyCode::Left => {
                    self.snake.update_direction(Direction::Left);
                }
                KeyCode::Char('d') | KeyCode::Char('l') | KeyCode::Right => {
                    self.snake.update_direction(Direction::Right);
                }
                _ => {},
            }
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        self.opration();
        self.snake.update_pos();
        self.food.update_food(self.window_width, self.window_height);
        self.check_game_over();
        self.check_eat_food();

        frame.render_widget(
            Block::new()
                .borders(Borders::all())
                .title(format!("<得分:  {}>", self.snake.body.len())),
            Rect::new(0, 0, self.window_width, self.window_height),
        );

        self.snake
            .body
            .iter()
            .for_each(|pos| frame.render_widget(BLOCK, Rect::new(pos.x, pos.y, 2, 1)));
        frame.render_widget(BLOCK, Rect::new(self.food.pos.x, self.food.pos.y, 2, 1));
        self.fps.do_sleep_time();
    }

    fn check_eat_food(&mut self) {
        let Self { snake, food, .. } = self;

        let head = snake.body.first().unwrap();
        let Pos { x, y } = food.pos;

        if head.y == food.pos.y && head.x == food.pos.x {
            food.alive = false;
            snake.body.insert(0, Pos { x, y });
        }
    }

    fn check_game_over(&mut self) {
        let Self { snake, .. } = self;
        let &Pos { x, y } = snake.body.first().unwrap();

        if x >= self.window_width || y >= self.window_height {
            self.is_over = true;
        } else if x <= 0 || y <= 0 {
            self.is_over = true;
        } else if snake
            .body
            .iter()
            .skip(1)
            .any(|pos| pos.x == x && pos.y == y)
        {
            self.is_over = true;
        }
    }
}
