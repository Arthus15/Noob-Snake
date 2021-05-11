pub use crate::game_core::{Direction, GameCore, Player};
pub use crate::gameboard_view::GameBoardView;

mod game_core;
mod gameboard_view;

fn main() {
    let mut game = GameBoardView::new();

    game.start();
}
