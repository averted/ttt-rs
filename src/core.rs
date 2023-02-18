use crate::position::Position;
use rand::Rng;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Turn {
    X,
    O,
}

impl Display for Turn {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
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

impl Turn {
    pub fn not(t: Turn) -> Self {
        match t {
            Turn::X => Turn::O,
            Turn::O => Turn::X,
        }
    }
}

pub fn get_rank_from_index(row_idx: usize) -> u8 {
    if row_idx > 2 {
        return 0;
    }

    (3 - row_idx) as u8
}

pub fn get_file_from_index(col_idx: usize) -> char {
    match col_idx {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        _ => '_',
    }
}

pub fn get_position_from_string(s: String) -> Result<Position, &'static str> {
    let file: char = s.chars().nth(0).unwrap();
    let rank: u8 = s.chars().nth(1).unwrap().to_digit(10).unwrap() as u8;

    if file <= 'c' && rank <= 3 {
        return Ok(Position::new(file, rank));
    }

    Err("Invalid position")
}

pub fn get_random_position() -> Position {
    let rand_file: u8 = 100 - rand::thread_rng().gen_range(1, 4);
    let rand_rank: u8 = rand::thread_rng().gen_range(1, 4);
    Position::new(rand_file as char, rand_rank)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank_conversion_works() {
        assert_eq!(get_rank_from_index(0), 3);
        assert_eq!(get_rank_from_index(1), 2);
        assert_eq!(get_rank_from_index(2), 1);
        assert_eq!(get_rank_from_index(3), 0);
        assert_eq!(get_rank_from_index(99), 0);
    }

    #[test]
    fn file_conversion_works() {
        assert_eq!(get_file_from_index(0), 'a');
        assert_eq!(get_file_from_index(1), 'b');
        assert_eq!(get_file_from_index(2), 'c');
        assert_eq!(get_file_from_index(3), '_');
        assert_eq!(get_file_from_index(99), '_');
    }

    /* TODO: assert fn to throw error??
    #[test]
    fn string_to_position_works() {
        assert_eq!(
            get_position_from_string(String::from("a1")),
            Position::new('a', 1)
        );

        assert_eq!(get_position_from_string(String::from("e1")), None);
    }
    */
}
