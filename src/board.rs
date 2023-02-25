use crate::player::Player;
use crate::position::Position;
use crate::renderer::Renderer;
use crate::square::Square;
use crate::BOARD_SIZE;

use std::collections::HashMap;
use std::io;

pub struct Board {
    pub player: Player,
    pub winner: Option<Player>,
    squares: Vec<Square>,
    renderer: Renderer,
}

impl Board {
    pub fn new() -> Self {
        Self {
            player: Player::X,
            squares: vec![],
            winner: None,
            renderer: Renderer::new(),
        }
    }

    #[cfg(test)]
    pub fn from(squares: Vec<Square>) -> Self {
        Self {
            player: Player::X,
            squares,
            winner: None,
            renderer: Renderer::new(),
        }
    }

    pub fn render(&self) {
        self.renderer.clear();
        self.renderer.draw(&self.squares);
    }

    pub fn move_to(&mut self, pos: Position) -> Result<Option<Player>, &'static str> {
        if self.is_move_available(&pos) {
            self.squares
                .push(Square::new(self.player, Position::new(pos.x, pos.y)));

            println!("Moving to: {}", pos);

            if self.check_for_win() {
                return Ok(self.winner);
            }

            self.player = Player::flip(self.player);
            return Ok(None);
        }

        Err("Move is not available")
    }

    pub fn ask_for_move(&self) -> Result<String, &'static str> {
        println!("Enter move:");

        let mut input = String::new();
        if let Err(_) = io::stdin().read_line(&mut input) {
            return Err("IO Error");
        }

        let input: String = input.trim().parse().unwrap();

        if input.len() != 2 {
            return Err("Invalid input length");
        }

        Ok(input)
    }

    pub fn make_ai_move(&mut self) -> Result<Option<Player>, &'static str> {
        let mut pos: Position = Position::rand();

        // TODO: Optimize
        while !self.is_move_available(&pos) {
            pos = Position::rand();
        }

        Ok(self.move_to(pos)?)
    }

    fn is_move_available(&self, pos: &Position) -> bool {
        !self.squares.iter().any(|square| square.at(&pos))
    }

    fn check_for_win(&mut self) -> bool {
        let arr: Vec<&Square> = self
            .squares
            .iter()
            .filter(|x| x.player == self.player)
            .collect();
        let mut map: HashMap<String, u8> = HashMap::new();

        if arr.len() < BOARD_SIZE {
            return false;
        }

        for sq in arr.iter() {
            let y_count = map.entry(format!("y-{}", sq.y())).or_insert(0);
            *y_count += 1;
        }

        for sq in arr.iter() {
            let x_count = map.entry(format!("x-{}", sq.x())).or_insert(0);
            *x_count += 1;
        }

        for (_, count) in &map {
            if usize::from(*count) == BOARD_SIZE {
                self.winner = Some(self.player);
                return true;
            }
        }

        let mut diagonal_win: bool = true;

        for i in 0..BOARD_SIZE {
            if diagonal_win
                && (!map.contains_key(&format!("y-{}", i))
                    || !map.contains_key(&format!("x-{}", i)))
            {
                diagonal_win = false;
            }
        }

        if diagonal_win {
            self.winner = Some(self.player);
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal_win_condition() {
        let mut squares = vec![];

        for i in 0..BOARD_SIZE {
            squares.push(Square::new(Player::X, Position::new(i, 0)));
        }

        let mut b = Board::from(squares);
        assert_eq!(b.check_for_win(), true);
        assert_eq!(b.winner, Some(Player::X));
    }

    #[test]
    fn vertical_win_condition() {
        let mut squares = vec![];
        for i in 0..BOARD_SIZE {
            squares.push(Square::new(Player::X, Position::new(0, i)));
        }

        let mut b = Board::from(squares);
        assert_eq!(b.check_for_win(), true);
        assert_eq!(b.winner, Some(Player::X));
    }

    #[test]
    fn diagonal_win_condition() {
        let mut squares = vec![];
        for i in 0..BOARD_SIZE {
            squares.push(Square::new(Player::X, Position::new(i, i)));
        }

        let mut b = Board::from(squares);
        assert_eq!(b.check_for_win(), true);
        assert_eq!(b.winner, Some(Player::X));

        let mut squares2 = vec![];
        for i in 0..BOARD_SIZE {
            squares2.push(Square::new(Player::X, Position::new(BOARD_SIZE - 1 - i, i)));
        }

        let mut b2 = Board::from(squares2);
        assert_eq!(b2.check_for_win(), true);
        assert_eq!(b2.winner, Some(Player::X));
    }
}
