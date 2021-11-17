use std::env;
mod pgn;
mod play;
mod search;
mod state;

fn main() {
    // Parse user arguments
    let args: Vec<String> = env::args().collect();
    if args.len() > 3 {
        invalid();
    } else {
        match args.len() {
            1 => play::vs_game(),
            2 => {
                if args[1] == "-help" {
                    help();
                } else {
                    invalid();
                }
            }
            3 => {
                if args[1] == "-random" && is_string_numeric(args[2].to_string()) {
                    play::ran_game(args[2].parse::<i32>().unwrap())
                } else {
                    invalid()
                }
            }
            _ => invalid(),
        }
    }
}

fn help() {
    println!("Available arguments:");
    println!("-random <num> | Plays <num> of random games.");
    println!("-help | Displays this help menu.");
}

fn invalid() {
    println!("Invalid arguments!");
    println!("Use '-help' to see valid arguments.");
}

fn is_string_numeric(str: String) -> bool {
    for c in str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}
