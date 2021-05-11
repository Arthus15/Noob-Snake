use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Clone, Debug)]
pub enum Player {
    Snake(Snake),
    Fruit(Position),
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

#[derive(Clone, Debug)]
pub struct Snake {
    pub position: Position,
    pub body: Vec<Position>,
    pub last_direction: Direction,
}

#[derive(Debug)]
pub struct GameCore {
    pub players: [Player; 2],
    pub score: u16,
}

impl Position {
    pub fn new() -> Position {
        Position { x: 0, y: 0 }
    }

    pub fn create(x: u16, y: u16) -> Position {
        Position { x, y }
    }

    pub fn random(x_max: u16, y_max: u16) -> Position {
        let x = (rand::thread_rng().gen_range(0..x_max) / 25) * 25;
        let y = (rand::thread_rng().gen_range(0..y_max) / 25) * 25;

        Position { x, y }
    }

    fn calculate_new_position(&mut self) {
        let new_pos = Position::random(800, 600);

        self.x = new_pos.x;
        self.y = new_pos.y;
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Position {}

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
            score: 0,
        }
    }

    pub fn move_snake(&mut self, dir: Direction) {
        let snake = self.get_snake_mut();
        match dir {
            Direction::Up => {
                if snake.position.y > 0 {
                    snake.position.y = snake.position.y - 25;
                    snake.last_direction = Direction::Up;
                }
            }
            Direction::Down => {
                if snake.position.y < 575 {
                    snake.position.y = snake.position.y + 25;
                    snake.last_direction = Direction::Down;
                }
            }
            Direction::Left => {
                if snake.position.x > 0 {
                    snake.position.x = snake.position.x - 25;
                    snake.last_direction = Direction::Left;
                }
            }
            Direction::Right => {
                if snake.position.x < 775 {
                    snake.position.x = snake.position.x + 25;
                    snake.last_direction = Direction::Right;
                }
            }
            Direction::None => (),
        }
    }

    pub fn eat_fruit(&mut self) {
        self.score += 1;

        let fruit = self.get_fruit_mut();

        fruit.calculate_new_position();
    }

    pub fn is_fruit_eatable(&self) -> bool {
        let fruit = self.get_fruit_unmut();
        let snake = self.get_snake_unmut();

        fruit == &snake.position
    }

    pub fn get_snake_unmut(&self) -> &Snake {
        let snake_enum = &self.players[0];
        if let Player::Snake(snake) = snake_enum {
            snake
        } else {
            panic!("No snake found!");
        }
    }

    fn get_snake_mut(&mut self) -> &mut Snake {
        let snake_enum = &mut self.players[0];
        if let Player::Snake(snake) = snake_enum {
            snake
        } else {
            panic!("No snake found!");
        }
    }

    fn get_fruit_mut(&mut self) -> &mut Position {
        let snake_enum = &mut self.players[1];
        if let Player::Fruit(pos) = snake_enum {
            pos
        } else {
            panic!("No fruit found!");
        }
    }

    fn get_fruit_unmut(&self) -> &Position {
        let snake_enum = &self.players[1];
        if let Player::Fruit(pos) = snake_enum {
            pos
        } else {
            panic!("No fruit found!");
        }
    }
}
