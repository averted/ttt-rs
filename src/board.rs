use crate::player::Player;
use crate::position::Position;
use crate::renderer::Renderer;
use crate::square::Square;

use std::collections::HashMap;
use std::io;

#[derive(Eq, Hash, PartialEq)]
enum Value {
    Char(char),
    Numb(u8),
}

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

    pub fn render(&self) {
        self.renderer.clear();
        self.renderer.draw(&self.squares);
    }

    pub fn move_to(&mut self, pos: Position) -> Result<Option<Player>, &'static str> {
        if self.is_move_available(&pos) {
            self.squares
                .push(Square::new(self.player, Position::from(&pos)));

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
        !self.squares.iter().any(|x| x.at(&pos))
    }

    fn check_for_win(&mut self) -> bool {
        let arr: Vec<&Square> = self
            .squares
            .iter()
            .filter(|x| x.player == self.player)
            .collect();
        let mut map: HashMap<Value, u8> = HashMap::new();

        if arr.len() <= 2 {
            return false;
        }

        for m in arr.iter() {
            let rank_count = map.entry(Value::Numb(m.rank())).or_insert(0);
            *rank_count += 1;
        }

        for m in arr.iter() {
            let file_count = map.entry(Value::Char(m.file())).or_insert(0);
            *file_count += 1;
        }

        for (_, count) in &map {
            if *count == 3 {
                self.winner = Some(self.player);
                return true;
            }
        }

        if map.contains_key(&Value::Numb(1))
            && map.contains_key(&Value::Numb(2))
            && map.contains_key(&Value::Numb(3))
            && map.contains_key(&Value::Char('a'))
            && map.contains_key(&Value::Char('b'))
            && map.contains_key(&Value::Char('c'))
            && arr.iter().any(|x| x.at(&Position::new('b', 2)))
        {
            self.winner = Some(self.player);
            return true;
        }

        false
    }
}
