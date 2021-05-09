pub struct Position {
    x: u8,
    y: u8,
}

pub enum Player {
    Snake(Position),
    Fruit(Position),
}
