use crate::player::Player;
use crate::position::Position;
use crate::square::Square;
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

    pub fn clear(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    pub fn draw(&self, squares: &Vec<Square>) {
        let border = String::from(" -----------");
        let legend = String::from("   a  b  c  ");
        let mut result = String::from(&border);

        for (row_idx, row) in self.matrix.iter().enumerate() {
            let mut top = " |".to_string();
            let mut mid = format!("{}|", 3 - (row_idx));
            let mut bot = " |".to_string();

            for (col_idx, _) in row.iter().enumerate() {
                let s = match Position::from_idx(col_idx, row_idx) {
                    Some(pos) => squares.iter().find(|x| x.at(&pos)),
                    None => None,
                };
                let (t, m, b) = &self.render_cell(s, row_idx, col_idx);

                top = format!("{}{}", top, t);
                mid = format!("{}{}", mid, m);
                bot = format!("{}{}", bot, b);
            }

            result = format!("{result}\n{top}|\n{mid}|\n{bot}|")
        }

        println!("{result}\n{border}\n{legend}")
    }

    fn render_cell(
        &self,
        square: Option<&Square>,
        row: usize,
        col: usize,
    ) -> (String, String, String) {
        let spacer = if (row + col) % 2 == 0 { "·" } else { " " };
        let buffer = format!("{}{}{}", spacer, spacer, spacer);

        (
            String::from(&buffer),
            match square {
                Some(mov) => {
                    format!(
                        "{}{}{}",
                        spacer,
                        if mov.player == Player::X {
                            mov.player.to_string().red()
                        } else {
                            mov.player.to_string().green()
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
