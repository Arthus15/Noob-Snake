pub use crate::game_core::{GameCore, Player};
pub use crate::gameboard_view::GameBoardView;

use piston_window::*;

//Constants
const WINDOWS_HEIGHT: f64 = 800.0;
const WINDOWS_WIDTH: f64 = 600.0;

mod game_core;
mod gameboard_view;

fn main() {
    let title = "Noob-Snake";
    let mut window: PistonWindow = WindowSettings::new(title, [WINDOWS_HEIGHT, WINDOWS_WIDTH])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let gameboard_view: GameBoardView = GameBoardView::new();

    window.set_lazy(true);
    let mut y = 775.0;
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear(gameboard_view.background_color, g);
            let snake_enum = &gameboard_view.game.players[0];
            if let Player::Snake(snake) = snake_enum {
                rectangle(
                    gameboard_view.snake_color,
                    [snake.position.x as f64, snake.position.y as f64, 25.0, 25.0],
                    c.transform,
                    g,
                );
            }

            let fruit_enum = &gameboard_view.game.players[1];
            if let Player::Fruit(pos) = fruit_enum {
                ellipse(
                    gameboard_view.fruit_color,
                    [pos.x as f64, pos.y as f64, 25.0, 25.0],
                    c.transform,
                    g,
                );
            }
        });

        match e.press_args() {
            Some(Button::Keyboard(Key::Left)) => y = y - 25.0,
            Some(Button::Keyboard(Key::Right)) => print!("RIGHT!"),
            Some(Button::Keyboard(Key::Up)) => print!("UP!"),
            Some(Button::Keyboard(Key::Down)) => print!("DOWN!"),
            _ => (),
        }
    }
}
