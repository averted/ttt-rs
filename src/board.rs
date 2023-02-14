use crate::core::Turn;
use crate::position::Position;
use crate::r#move::Move;
use crate::renderer::Renderer;
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
enum Value {
    Char(char),
    Numb(u8),
}

pub struct Board {
    pub turn: Turn,
    pub winner: Option<Turn>,
    moves: Vec<Move>,
    renderer: Renderer,
}

impl Board {
    pub fn new() -> Self {
        Self {
            turn: Turn::X,
            moves: vec![],
            winner: None,
            renderer: Renderer::new(),
        }
    }

    pub fn render(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{}", self.renderer.render(&self.moves));
    }

    fn is_available(&self, pos: &Position) -> bool {
        !self.moves.iter().any(|x| x.at(&pos))
    }

    fn check_for_win(&mut self) {
        let mut arr = vec![];
        let mut map: HashMap<Value, u8> = HashMap::new();

        // filter moves for current turn
        for i in 0..self.moves.len() {
            let m = &self.moves[i];

            if m.turn == self.turn {
                arr.push(m)
            }
        }

        if arr.len() <= 2 {
            return;
        }

        for m in arr.iter() {
            let rank_count = map.entry(Value::Numb(m.rank())).or_insert(0);
            *rank_count += 1;
        }

        for m in arr.iter() {
            let file_count = map.entry(Value::Char(m.file())).or_insert(0);
            *file_count += 1;
        }

        for (key, count) in &map {
            if *count == 3 {
                self.winner = Some(self.turn);
                return;
            }
        }

        if map.contains_key(&Value::Numb(1))
            && map.contains_key(&Value::Numb(2))
            && map.contains_key(&Value::Numb(3))
            && map.contains_key(&Value::Char('a'))
            && map.contains_key(&Value::Char('b'))
            && map.contains_key(&Value::Char('c'))
        {
            self.winner = Some(self.turn);
            return;
        }
    }

    pub fn move_to(&mut self, pos: Position) -> Result<(), String> {
        if self.is_available(&pos) {
            self.moves.push(Move::new(self.turn, Position::from(&pos)));

            println!("Moving to: {}", pos);

            // check win condition and update winner
            self.check_for_win();

            return Ok(());
        }

        Err(String::from("Move is not available"))
    }

    pub fn flip_turn(&mut self) {
        if self.turn == Turn::X {
            self.turn = Turn::O;
        } else {
            self.turn = Turn::X;
        }
    }

    pub fn get_remaining_moves(&self) -> usize {
        if let Some(winner) = &self.winner {
            return 0;
        }

        9 - self.moves.len()
    }
}
