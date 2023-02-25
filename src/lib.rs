mod board;
mod player;
mod position;
mod renderer;
mod square;

use crate::board::Board;
use crate::player::Player;
use crate::position::Position;

static BOARD_SIZE: usize = 10;

pub fn run() -> Result<Option<Player>, &'static str> {
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
