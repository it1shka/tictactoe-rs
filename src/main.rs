mod game;
use game::Game;
use std::io::stdin;

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

fn main() {
    let mut game_state = Game::new();
    let mut error: Option<String> = None;
    while game_state.status == None && game_state.free_space > 0 {
        clear_terminal!();
        println!("{}", game_state.display());
        if let Some(err) = error {
            println!("{}", err);
            error = None
        }
        match get_user_input() {
            Err(some_error) => {
                error = Some(some_error)
            },
            Ok((row, col)) => {
                match game_state.place_figure(row, col) {
                    Ok(next) => {
                        game_state = next
                    },
                    Err(some_error) => {
                        error = Some(some_error)
                    }
                }
            }
        }
    }
    println!("{}", game_state.display_game_result())
}
