use std::process;

fn main() {
    match ttt::run() {
        Ok(winner) => {
            if let Some(winner) = winner {
                println!("Winner: {winner}");
            } else {
                println!("Tie");
            }
        }
        Err(e) => {
            eprintln!("Application error: {e}");
            process::exit(1);
        }
    }
}
