use crate::position::Position;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Turn {
    X,
    O,
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

pub fn get_rank_from_index(row_idx: usize) -> u8 {
    (3 - row_idx) as u8
}

pub fn get_file_from_index(col_idx: usize) -> char {
    match col_idx {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        _ => 'x',
    }
}

pub fn get_position_from_string(s: String) -> Option<Position> {
    let file: char = s.chars().nth(0).unwrap();
    let rank: u8 = s.chars().nth(1).unwrap().to_digit(10).unwrap() as u8;

    if file <= 'c' && rank <= 3 {
        return Some(Position::new(file, rank));
    }

    None
}
