use piston_window::*;

use crate::game_core::{Direction, GameCore, Player};

#[derive(Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

//class for build the screen
#[derive(Debug)]
pub struct GameBoardView {
    pub background_color: types::Color,
    pub snake_color: types::Color,
    pub fruit_color: types::Color,
    pub game: GameCore,
    pub title: String,
    pub difficulty: Difficulty,
}

impl GameBoardView {
    pub fn new() -> GameBoardView {
        GameBoardView {
            background_color: [0.5, 1.0, 0.5, 1.0],
            snake_color: [1.0, 0.0, 1.0, 0.5],
            fruit_color: [1.0, 0.4, 0.1, 1.0],
            title: String::from("Noob-Snake"),
            difficulty: Difficulty::Hard,
            game: GameCore::new(),
        }
    }

    pub fn start(&mut self) {
        //Windows setup
        let mut window: PistonWindow = self.create_piston_window();
        let mut refresh_per_move: u16 = 0;

        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g, _| {
                clear(self.background_color, g);
                let snake_enum = &self.game.players[0];
                if let Player::Snake(snake) = snake_enum {
                    rectangle(
                        self.snake_color,
                        [snake.position.x as f64, snake.position.y as f64, 25.0, 25.0],
                        c.transform,
                        g,
                    );

                    if snake.body.len() > 0 {
                        for n in 0..(snake.body.len()) {
                            let body_part = snake.body[n];
                            rectangle(
                                [0.0, 0.0, 1.0, 1.0],
                                [body_part.0.x as f64, body_part.0.y as f64, 25.0, 25.0],
                                c.transform,
                                g,
                            );
                        }
                    }
                }

                let fruit_enum = &self.game.players[1];
                if let Player::Fruit(pos) = fruit_enum {
                    ellipse(
                        self.fruit_color,
                        [pos.x as f64, pos.y as f64, 25.0, 25.0],
                        c.transform,
                        g,
                    );
                }
            });

            match e.press_args() {
                Some(Button::Keyboard(Key::Left)) => {
                    self.game.move_snake(Direction::Left, &mut refresh_per_move);
                }
                Some(Button::Keyboard(Key::Right)) => {
                    self.game
                        .move_snake(Direction::Right, &mut refresh_per_move);
                }
                Some(Button::Keyboard(Key::Up)) => {
                    self.game.move_snake(Direction::Up, &mut refresh_per_move);
                }
                Some(Button::Keyboard(Key::Down)) => {
                    self.game.move_snake(Direction::Down, &mut refresh_per_move);
                }
                _ => {
                    if refresh_per_move >= 100 {
                        self.game.move_snake(
                            self.game.get_snake_unmut().last_direction,
                            &mut refresh_per_move,
                        );
                    } else {
                        refresh_per_move += 1;
                    }
                }
            }

            if self.game.is_fruit_eatable() {
                self.game.eat_fruit();
                self.game.add_tail();
            }
        }
    }

    fn create_piston_window(&self) -> PistonWindow {
        let mut window: PistonWindow = WindowSettings::new(self.title.to_string(), [800, 600])
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

        window.set_lazy(false);

        window
    }
}
