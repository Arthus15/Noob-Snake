use piston_window::*;

use crate::game_core::{Direction, GameCore, Player};

//class for build the screen
#[derive(Debug)]
pub struct GameBoardView {
    pub background_color: types::Color,
    pub snake_color: types::Color,
    pub fruit_color: types::Color,
    pub game: GameCore,
}

impl GameBoardView {
    pub fn new() -> GameBoardView {
        GameBoardView {
            background_color: [0.5, 1.0, 0.5, 1.0],
            snake_color: [1.0, 0.0, 1.0, 0.5],
            fruit_color: [1.0, 0.4, 0.1, 1.0],
            game: GameCore::new(),
        }
    }

    pub fn start(&mut self) {
        let title = "Noob-Snake";
        let mut window: PistonWindow = WindowSettings::new(title, [800, 600])
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

        let mut gameboard_view: GameBoardView = GameBoardView::new();
        let mut refresh_per_move = 0;
        window.set_lazy(false);
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
                Some(Button::Keyboard(Key::Left)) => {
                    gameboard_view.game.move_snake(Direction::Left);
                    refresh_per_move = 0;
                }
                Some(Button::Keyboard(Key::Right)) => {
                    gameboard_view.game.move_snake(Direction::Right);
                    refresh_per_move = 0;
                }
                Some(Button::Keyboard(Key::Up)) => {
                    gameboard_view.game.move_snake(Direction::Up);
                    refresh_per_move = 0;
                }
                Some(Button::Keyboard(Key::Down)) => {
                    gameboard_view.game.move_snake(Direction::Down);
                    refresh_per_move = 0;
                }
                _ => {
                    if refresh_per_move >= 200 {
                        gameboard_view
                            .game
                            .move_snake(gameboard_view.game.get_snake_unmut().last_direction);
                        refresh_per_move = 0;
                    } else {
                        refresh_per_move += 1;
                    }
                }
            }

            if gameboard_view.game.is_fruit_eatable() {
                gameboard_view.game.eat_fruit();
            }
        }
    }
}
