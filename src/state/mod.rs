use pleco::Board;
mod insufficient;
mod three_fold;

pub fn is_game_over(board: &Board, positions: &Vec<u64>) -> bool {
    if !board.checkmate()
        && !board.stalemate()
        && !insufficient::is_insufficient_material(board)
        && !three_fold::is_three_fold(board, positions)
    {
        return false;
    }

    true
}

pub fn get_positions(board: &Board, positions: Vec<u64>) -> Vec<u64> {
    let new_val: Vec<u64> = vec![board.zobrist()];
    if board.rule_50() >= 1 {
        [positions, new_val].concat()
    } else {
        vec![]
    }
}
