mod board;
mod r#move;
mod position;
mod renderer;
mod turn;

use crate::board::Board;
use crate::position::Position;
use crate::turn::Turn;

pub fn run() -> Result<Option<Turn>, &'static str> {
    let mut board = Board::new();

    while board.winner == None {
        board.render();

        match board.move_to(Position::from_str(board.ask_for_move()?)?) {
            Ok(opt) => {
                if let Some(_) = opt {
                    continue;
                }
            }
            Err(err) => {
                println!("{err}");
                continue;
            }
        };

        board.make_ai_move()?;
    }

    board.render();
    Ok(board.winner)
}
