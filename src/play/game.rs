use crate::pgn;
use crate::search;
use crate::state;
use pleco::{Board, Player};
use rand::Rng;
use std::io::stdin;

pub fn play_game(random: bool) {
    // Reset board
    let mut board = Board::start_pos();

    // Tracks positions in game
    let mut positions: Vec<u64> = vec![];
    let mut pgn = "".to_string();

    // Play game
    while !state::is_game_over(&board, &positions) {
        // Get legal moves for position
        let legal_moves = board.generate_moves();

        if random {
            // Make best move
            // board.apply_move(search::get_best_move(&board));

            // Make random move
            let mut rng = rand::thread_rng();
            let ran_move_num = rng.gen_range(0..legal_moves.len());
            board.apply_move(legal_moves[ran_move_num]);
        } else {
            if board.turn() == Player::White {
                // Show board
                board.pretty_print();

                // Show legal moves
                for i in 0..legal_moves.len() - 1 {
                    if i == legal_moves.len() - 2 {
                        print!("{} = {}", i + 1, legal_moves[i]);
                    } else {
                        print!("{} = {}, ", i + 1, legal_moves[i]);
                    }
                }
                println!("");

                // Get user move
                let mut player_move = String::new();
                stdin().read_line(&mut player_move).unwrap();
                player_move.pop();

                // Play user move
                board.apply_move(legal_moves[player_move.parse::<usize>().unwrap() - 1]);
                println!("");
            } else {
                // Make best move
                board.apply_move(search::get_best_move(&board));

                // Make random move
                // let mut rng = rand::thread_rng();
                // let ran_move_num = rng.gen_range(0..legal_moves.len());
                // board.apply_move(legal_moves[ran_move_num]);
            }
        }

        // Keeping track of PGN
        pgn = pgn::get_pgn(&board, pgn);

        // Keeping track of positions
        positions = state::get_positions(&board, positions);
    }

    // Print results
    board.pretty_print();
    println!("{}", pgn);
    println!("Moves: {}", board.moves_played() / 2);
}
