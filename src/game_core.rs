use rand::Rng;

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Clone)]
pub enum Player {
    Snake(Snake),
    Fruit(Position),
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

#[derive(Clone)]
pub struct Snake {
    pub position: Position,
    pub body: Vec<Position>,
    pub last_direction: Direction,
}

pub struct GameCore {
    pub players: [Player; 2],
}

impl Position {
    pub fn new() -> Position {
        Position { x: 0, y: 0 }
    }

    pub fn create(x: u16, y: u16) -> Position {
        Position { x, y }
    }

    pub fn random(x_max: u16, y_max: u16) -> Position {
        let x = rand::thread_rng().gen_range(0..x_max) / 25;
        let y = rand::thread_rng().gen_range(0..y_max) / 25;

        Position { x, y }
    }
}

impl Snake {
    pub fn new(x: u16, y: u16) -> Snake {
        let head_position = Position::create(x, y);

        Snake {
            position: head_position,
            body: vec![head_position],
            last_direction: Direction::None,
        }
    }
}

impl GameCore {
    pub fn new() -> GameCore {
        GameCore {
            players: [
                Player::Snake(Snake::new(0, 0)),
                Player::Fruit(Position::random(800, 600)),
            ],
        }
    }
}
