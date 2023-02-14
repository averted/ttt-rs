use std::fmt::{Display, Formatter, Result};

pub struct Position {
    pub file: char,
    pub rank: u8,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Position({}-{})", self.file, self.rank)
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.file == other.file && self.rank == other.rank
    }
}

impl Position {
    pub fn new(file: char, rank: u8) -> Self {
        Self { file, rank }
    }

    pub fn from(pos: &Position) -> Self {
        Self {
            file: pos.file,
            rank: pos.rank,
        }
    }
}
