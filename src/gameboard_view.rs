use piston_window::*;

use crate::game_core::GameCore;

//class for build the screen
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
}
