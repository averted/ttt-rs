use crate::BOARD_SIZE;
use rand::Rng;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Position({}-{})", self.x, self.y)
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn rand() -> Self {
        let x: usize = rand::thread_rng().gen_range(0, BOARD_SIZE);
        let y: usize = rand::thread_rng().gen_range(0, BOARD_SIZE);

        Self { x, y }
    }

    pub fn from_str(s: String) -> Result<Self, &'static str> {
        let x: usize = match s.chars().nth(0).unwrap().to_digit(10) {
            Some(x) => x as usize,
            None => return Err("Invalid position"),
        };

        let y: usize = match s.chars().nth(1).unwrap().to_digit(10) {
            Some(y) => y as usize,
            None => return Err("Invalid position"),
        };

        if x > BOARD_SIZE - 1 || y > BOARD_SIZE - 1 {
            return Err("Invalid position");
        }

        Ok(Self { x, y })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_position_from_valid_string() {
        assert_eq!(
            Position::from_str(String::from("00")),
            Ok(Position::new(0, 0))
        );

        assert_eq!(
            Position::from_str(String::from("11")),
            Ok(Position::new(1, 1))
        );

        assert_eq!(
            Position::from_str(String::from("22")),
            Ok(Position::new(2, 2))
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
