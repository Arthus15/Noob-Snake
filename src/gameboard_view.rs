use piston_window::*;

//Constants
const WINDOWS_HEIGHT: f64 = 800.0;
const WINDOWS_WIDTH: f64 = 600.0;

//class for build the screen
pub struct GameBoardView {
    pub background_color: types::Color,
    pub snake_color: types::Color,
}

impl GameBoardView {
    pub fn new() -> GameBoardView {
        GameBoardView {
            background_color: [0.5, 1.0, 0.5, 1.0],
            snake_color: [1.0, 0.0, 1.0, 0.5],
        }
    }

    pub fn game_init(&self) {
        let title = "Noob-Snake";
        let mut window: PistonWindow = WindowSettings::new(title, [WINDOWS_HEIGHT, WINDOWS_WIDTH])
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

        print!("size -> {:?}", window.size());

        window.set_lazy(true);
        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g, _| {
                clear(self.background_color, g);
                rectangle(self.snake_color, [775.0, 0.0, 25.0, 25.0], c.transform, g);
            });
        }
    }
}
