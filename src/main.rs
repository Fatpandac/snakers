mod fps;
mod food;
mod game;
mod share;
mod snake;

use game::Game;

fn main() {
    let mut terminal = ratatui::init();
    let mut game = Game::new(&mut terminal);

    loop {
        terminal.draw(|frame| game.render(frame)).unwrap();
        if game.is_over {
            break;
        }
    }
    ratatui::restore();
}
