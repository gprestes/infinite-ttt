mod game;

use game::Game;
use std::io::{self, Write};

fn print_board(game: &Game) {
    for row in 0..3 {
        for col in 0..3 {
            let symbol = match game.board[row][col] {
                game::Cell::Empty => ".",
                game::Cell::Occupied(game::Player::X) => "X",
                game::Cell::Occupied(game::Player::O) => "O",
            };
            print!("{} ", symbol);
        }
        println!();
    }
}

fn main() {
    let mut game = Game::new();

    println!("Welcome to Infinite Tic Tac Toe!");

    while game.winner.is_none() {
        print_board(&game);

        if game.current_player == game::Player::X {
            println!("Player X, type line and column  (0-2):");

            let mut input = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();

            let coords: Vec<usize> = input
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            if coords.len() == 2 {
                match game.play(coords[0], coords[1]) {
                    Ok(_) => continue,
                    Err(msg) => println!("Error: {}", msg),
                }
            } else {
                println!("Invalid input. Type two numbers.");
            }
        } else {
            println!("AI (Player O) is playing...");
            game.play_ai_move().unwrap();
        }
    }

    print_board(&game);
    println!("Player {:?} won!", game.winner.unwrap());
}
