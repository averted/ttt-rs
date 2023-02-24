use crate::player::Player;
use crate::position::Position;

pub struct Square {
    pub player: Player,
    position: Position,
}

impl Square {
    pub fn new(player: Player, position: Position) -> Self {
        Self { player, position }
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
