use pleco::{Board, PieceType};

pub fn uci_to_pgn(board: &Board, piece: PieceType) -> String {
    if piece.is_some() {
        let move_dest = board.last_move().unwrap().get_dest();
        let move_uci = board.last_move().unwrap().stringify();
        let piece_type = piece.char_upper();
        let is_promo = board.last_move().unwrap().is_promo();

        if is_promo {
            format!("{}={}", move_dest, piece_type).to_string()
        } else {
            match piece_type {
                'P' => format!("{}", move_dest).to_string(),
                'K' => format!("{}{}", piece_type, move_dest).to_string(),
                _ => format!("{}{}", piece_type, move_uci).to_string(),
            }
        }
    } else {
        let is_king_castle = board.last_move().unwrap().is_king_castle();

        if is_king_castle {
            "O-O".to_string()
        } else {
            "O-O-O".to_string()
        }
    }
}
