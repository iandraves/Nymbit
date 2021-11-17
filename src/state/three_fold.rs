use pleco::Board;

pub fn is_three_fold(board: &Board, positions: &Vec<u64>) -> bool {
    if positions.iter().filter(|&n| *n == board.zobrist()).count() == 3 {
        true;
    }
    false
}
