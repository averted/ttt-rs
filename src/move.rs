use crate::core::Turn;
use crate::position::Position;

pub struct Move {
    pub turn: Turn,
    position: Position,
}

impl Move {
    pub fn new(turn: Turn, position: Position) -> Self {
        Self { turn, position }
    }

    pub fn at(&self, position: &Position) -> bool {
        self.position == *position
    }

    pub fn rank(&self) -> u8 {
        self.position.rank
    }

    pub fn file(&self) -> char {
        self.position.file
    }
}
