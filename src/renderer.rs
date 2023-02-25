use crate::player::Player;
use crate::position::Position;
use crate::square::Square;
use crate::BOARD_SIZE;
use colored::Colorize;

pub struct Renderer {
    matrix: Vec<Vec<usize>>,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            matrix: vec![vec![0; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    pub fn clear(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    pub fn draw(&self, squares: &Vec<Square>) {
        let mut border = String::new();
        let mut legend = String::new();

        for i in 0..BOARD_SIZE {
            border.push_str("---");
            legend.push_str(&format!(" {} ", i));
        }

        let border = format!(" -{border}-");
        let legend = format!("  {legend} ");
        let mut result = String::new();

        for (row_idx, row) in self.matrix.iter().enumerate() {
            let mut top = " |".to_string();
            let mut mid = format!("{}|", row_idx);
            let mut bot = " |".to_string();

            for (col_idx, _) in row.iter().enumerate() {
                let pos = Position::new(col_idx, row_idx);
                let s = squares.iter().find(|x| x.at(&pos));
                let (t, m, b) = &self.render_cell(s, row_idx, col_idx);

                top = format!("{}{}", top, t);
                mid = format!("{}{}", mid, m);
                bot = format!("{}{}", bot, b);
            }

            result = format!("{result}\n{top}|\n{mid}|\n{bot}|")
        }

        println!("{legend}\n{border}{result}\n{border}")
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
