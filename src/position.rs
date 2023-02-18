use rand::Rng;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct Position {
    pub file: char,
    pub rank: u8,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
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

    pub fn rand() -> Self {
        let file: u8 = 100 - rand::thread_rng().gen_range(1, 4);
        let rank: u8 = rand::thread_rng().gen_range(1, 4);

        Self {
            file: file as char,
            rank,
        }
    }

    pub fn from(pos: &Position) -> Self {
        Self {
            file: pos.file,
            rank: pos.rank,
        }
    }

    pub fn from_str(s: String) -> Result<Self, &'static str> {
        let file: char = s.chars().nth(0).unwrap();
        let rank: u8 = match s.chars().nth(1).unwrap().to_digit(10) {
            Some(rank) => rank as u8,
            None => return Err("Invalid position"),
        };

        if file > 'c' || file < 'a' || rank > 3 || rank < 1 {
            return Err("Invalid position");
        }

        Ok(Self { file, rank })
    }

    pub fn from_idx(col_idx: usize, row_idx: usize) -> Option<Self> {
        let file = match col_idx {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            _ => return None,
        };

        let rank = if row_idx > 2 {
            return None;
        } else {
            (3 - row_idx) as u8
        };

        Some(Self { file, rank })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_position_from_valid_idx() {
        assert_eq!(Position::from_idx(0, 0), Some(Position::new('a', 3)));
        assert_eq!(Position::from_idx(1, 1), Some(Position::new('b', 2)));
        assert_eq!(Position::from_idx(2, 2), Some(Position::new('c', 1)));
    }

    #[test]
    fn correct_position_from_invalid_idx() {
        assert_eq!(Position::from_idx(99, 0), None);
        assert_eq!(Position::from_idx(3, 0), None);
        assert_eq!(Position::from_idx(0, 99), None);
        assert_eq!(Position::from_idx(0, 3), None);
    }

    #[test]
    fn correct_position_from_valid_string() {
        assert_eq!(
            Position::from_str(String::from("a1")),
            Ok(Position::new('a', 1))
        );

        assert_eq!(
            Position::from_str(String::from("b2")),
            Ok(Position::new('b', 2))
        );

        assert_eq!(
            Position::from_str(String::from("c3")),
            Ok(Position::new('c', 3))
        );
    }

    #[test]
    fn errors_out_on_invalid_string_position() {
        assert!(Position::from_str(String::from("d1")).is_err());
        assert!(Position::from_str(String::from("∆1")).is_err());
        assert!(Position::from_str(String::from("a4")).is_err());
        assert!(Position::from_str(String::from("aa")).is_err());
        assert!(Position::from_str(String::from("aaa")).is_err());
        assert!(Position::from_str(String::from("a∆")).is_err());
        assert!(Position::from_str(String::from("∆∆∆")).is_err());
    }
}
