use pleco::Board;

pub fn is_insufficient_material(board: &Board) -> bool {
    // Material keys
    let insufficient_material_keys: Vec<u64> = vec![
        1440794681747902690,  // k vs. K
        12777358361509687304, // k & n vs. K
        14531074927864995568, // k vs. K & N
        15001244464087343965, // k & b vs. K
        2182324930161781490,  // k vs. K & B
        15967714286867192141, // k & b vs. K & B (bishops on same color)
    ];

    if insufficient_material_keys.contains(&board.material_key()) {
        return true;
    }

    false
}
