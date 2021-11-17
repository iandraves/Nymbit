use pleco::Board;
mod convert;

pub fn get_pgn(board: &Board, pgn: String) -> String {
    let num_of_moves = board.moves_played();
    let piece = board
        .piece_at_sq(board.last_move().unwrap().get_dest())
        .type_of();

    if num_of_moves % 2 == 0 {
        pgn + &*format!(" {} ", convert::uci_to_pgn(board, piece))
    } else {
        pgn + &*format!(
            "{}. {}",
            (num_of_moves / 2) + 1,
            convert::uci_to_pgn(board, piece)
        )
    }
}
