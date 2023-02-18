mod board;
mod core;
mod r#move;
mod position;
mod renderer;

use crate::board::Board;

pub fn run() -> Result<Option<core::Turn>, &'static str> {
    let mut board = Board::new();

    while board.get_remaining_moves() > 0 {
        board.render();

        match board.move_to(core::get_position_from_string(board.ask_for_move()?)?) {
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
