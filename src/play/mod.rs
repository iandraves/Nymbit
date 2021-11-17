mod game;

pub fn vs_game() {
    game::play_game(false);
}

pub fn ran_game(num_of_games: i32) {
    for i in 0..num_of_games {
        game::play_game(true);
        println!("Game: {}\n", i + 1);
        println!("----------------------------\n")
    }
}
