mod game;
mod computer;

use game::Game;
use std::env;
use std::{io::stdin, thread};
use std::time::Duration;

macro_rules! clear_terminal {
    () => {
        print!("\x1B[2J\x1B[1;1H");
    };
}

fn get_user_input() -> Result<(usize, usize), String> {
    println!("Enter your move like (row, col): ");
    let mut raw_input = String::new();
    if let Err(_) = stdin().read_line(&mut raw_input) {
        return Err(String::from("Failed to read input from stdin!"))
    }
    let integers = raw_input
        .trim()
        .split(" ")
        .map(|e| e.parse::<usize>())
        .collect::<Vec<_>>();
    match integers[..] {
        [Ok(a), Ok(b)] => Ok((a, b)),
        _ =>  Err(String::from("Input format is wrong: expected two integers!"))
    }
    
}

fn game_loop(computer: bool) {
    let mut game = Game::new();
    let mut error: Option<String> = None;

    loop {
        clear_terminal!();
        println!("{}", game.display());
        if game.free_space == 0 || game.status != None {
            break
        }
        println!("{}", game.display_current_turn());
        if let Some(err) = error {
            println!("{}", err);
            error = None
        }
        match get_user_input() {
            Err(some_error) => { error = Some(some_error) },
            Ok((user_row, user_col)) => {
                match game.place_figure(user_row, user_col) {
                    Ok(next) => {
                        game = next;
                        if computer && game.status == None && game.free_space > 0 { 
                            clear_terminal!();
                            println!("{}", game.display());
                            println!("{}", game.display_current_turn());
                            thread::sleep(Duration::from_millis(1000));
                            game = game.find_best_move().expect("Computer failed!") 
                        }
                    },
                    Err(some_error) => { error = Some(some_error) }
                }
            }
        }
    }
    println!("{}", game.display_game_result())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    game_loop(args.len() > 1 && args[1] == "ai")
}
