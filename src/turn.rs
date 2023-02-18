use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Turn {
    X,
    O,
}

impl Turn {
    pub fn flip(t: Turn) -> Self {
        match t {
            Turn::X => Turn::O,
            Turn::O => Turn::X,
        }
    }
}

impl Display for Turn {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Turn::X => {
                write!(f, "X")
            }
            Turn::O => {
                write!(f, "O")
            }
        }
    }
}
