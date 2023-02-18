use crate::position::Position;
use crate::r#move::Move;
use crate::turn::Turn;
use colored::*;

pub struct Renderer {
    matrix: [[u8; 3]; 3],
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            matrix: [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
        }
    }

    pub fn render(&self, moves: &Vec<Move>) -> String {
        let border = String::from(" -----------");
        let legend = String::from("   a  b  c  ");
        let mut result = String::from(&border);

        for (row_idx, row) in self.matrix.iter().enumerate() {
            let mut top = " |".to_string();
            let mut mid = format!("{}|", 3 - (row_idx));
            let mut bot = " |".to_string();

            for (col_idx, _) in row.iter().enumerate() {
                let m = match Position::from_idx(col_idx, row_idx) {
                    Some(pos) => moves.iter().find(|x| x.at(&pos)),
                    None => None,
                };
                let (t, m, b) = &self.render_cell(m, row_idx, col_idx);

                top = format!("{}{}", top, t);
                mid = format!("{}{}", mid, m);
                bot = format!("{}{}", bot, b);
            }

            result = format!("{result}\n{top}|\n{mid}|\n{bot}|")
        }

        format!("{result}\n{border}\n{legend}")
    }

    fn render_cell(&self, m: Option<&Move>, row: usize, col: usize) -> (String, String, String) {
        let spacer = if (row + col) % 2 == 0 { "·" } else { " " };
        let buffer = format!("{}{}{}", spacer, spacer, spacer);

        (
            String::from(&buffer),
            match m {
                Some(mov) => {
                    format!(
                        "{}{}{}",
                        spacer,
                        if mov.turn == Turn::X {
                            mov.turn.to_string().red()
                        } else {
                            mov.turn.to_string().green()
                        },
                        spacer
                    )
                }
                None => String::from(&buffer),
            },
            String::from(&buffer),
        )
    }
}
