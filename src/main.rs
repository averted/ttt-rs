mod board;
mod core;
mod r#move;
mod position;
mod renderer;

use crate::board::Board;
use crate::position::Position;
use std::io;

fn main() {
    let mut board = Board::new();
    board.render();

    while board.get_remaining_moves() > 0 {
        println!("Enter move ({:?}):", board.turn);

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read");

        let input: String = match input.trim().parse() {
            Ok(val) => val,
            Err(_) => continue,
        };

        if input.len() != 2 {
            continue;
        }

        if let Some(pos) = core::get_position_from_string(input) {
            match board.move_to(pos) {
                Ok(_) => {
                    board.flip_turn();
                    board.render();
                }
                Err(err) => {
                    println!("{}", err);
                }
            }
        } else {
            println!("Invalid position");
        }
    }

    if let Some(winner) = board.winner {
        println!("WINNER: {:?}", winner);
    } else {
        println!("TIE");
    }
}
