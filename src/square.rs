use crate::player::Player;
use crate::position::Position;

#[derive(Debug)]
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

    pub fn x(&self) -> usize {
        self.position.x
    }

    pub fn y(&self) -> usize {
        self.position.y
    }
}
