use rand::Rng;

#[derive(Copy, Clone, Debug, PartialEq)]
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
    pub body: Vec<(Position, Direction)>,
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

    pub fn _move(&mut self, dir: Direction) -> bool {
        match dir {
            Direction::Up => {
                self.y = self.y - 25;
            }
            Direction::Down => {
                self.y = self.y + 25;
            }
            Direction::Left => {
                self.x = self.x - 25;
            }
            Direction::Right => {
                self.x = self.x + 25;
            }
            Direction::None => {
                return false;
            }
        };

        true
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Position {}

impl Direction {
    fn oposite(self, b: Direction) -> bool {
        match self {
            Direction::Up => {
                if b == Direction::Down {
                    return true;
                }
                return false;
            }
            Direction::Down => {
                if b == Direction::Up {
                    return true;
                }

                return false;
            }
            Direction::Left => {
                if b == Direction::Right {
                    return true;
                }

                return false;
            }
            Direction::Right => {
                if b == Direction::Left {
                    return true;
                }

                return false;
            }
            Direction::None => {
                return false;
            }
        };
    }

    fn get_oposite(self) -> Direction {
        return match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::None => Direction::None,
        };
    }
}

impl Snake {
    pub fn new(x: u16, y: u16) -> Snake {
        let head_position = Position::create(x, y);

        Snake {
            position: head_position,
            body: vec![],
            last_direction: Direction::None,
        }
    }
}

fn valid_position(snake_pos: Position, dir: Direction) -> bool {
    let mut pos = snake_pos;

    pos._move(dir);

    if pos.y > 575 || pos.x > 775 {
        return false;
    }

    true
}

pub fn get_new_tail(old_tail: (Position, Direction)) -> (Position, Direction) {
    let mut new_tail = (old_tail.0, old_tail.1);
    let oposite = old_tail.1.get_oposite();

    new_tail.0._move(oposite);

    new_tail
}

impl GameCore {
    pub fn new() -> GameCore {
        GameCore {
            players: [
                Player::Snake(Snake::new(375, 275)),
                Player::Fruit(Position::random(800, 600)),
            ],
            score: 0,
        }
    }

    pub fn move_snake(&mut self, dir: Direction, _refresh_auto_move: &mut u16) {
        let snake = self.get_snake_mut();

        if dir.oposite(snake.last_direction) {
            return;
        }

        if !valid_position(snake.position, dir) {
            panic!("You've lost!");
        }

        snake.position._move(dir);
        snake.last_direction = dir;
        *_refresh_auto_move = 0;
        if snake.body.len() > 0 {
            self.move_snake_body();
        }
    }

    pub fn move_snake_body(&mut self) {
        let snake = self.get_snake_mut();
        for n in (0..snake.body.len()).rev() {
            let mut body_part = snake.body[n];
            body_part.0._move(body_part.1);

            if n == 0 {
                body_part.1 = snake.last_direction;
            } else {
                let direction = snake.body[n - 1].1;
                body_part.1 = direction;
            }

            snake.body.remove(n);
            snake.body.insert(n, body_part);
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

    pub fn add_tail(&mut self) {
        let snake = self.get_snake_mut();
        let body_length = snake.body.len();
        let old_tail;

        if body_length > 0 {
            old_tail = snake.body[body_length - 1];
        } else {
            old_tail = (snake.position, snake.last_direction);
        }

        let new_tail = get_new_tail(old_tail);

        snake.body.insert(body_length, new_tail);
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
