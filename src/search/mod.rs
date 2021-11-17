use pleco::{BitMove, Board};
mod eval;

pub fn get_best_move(board: &Board) -> BitMove {
    // Get legal moves for position
    let legal_moves = board.generate_moves();

    eval::evaluate_position(board);
    // for mv in legal_moves.iter() {
    //     let possible_position = *board;
    //     possible_position.apply_move(mv);
    //     eval::evaluate_position(possible_position);
    // }

    legal_moves[0]
}
