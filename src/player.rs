use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn flip(player: Player) -> Self {
        match player {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Player::X => {
                write!(f, "X")
            }
            Player::O => {
                write!(f, "O")
            }
        }
    }
}
