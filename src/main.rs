mod board;
// mod bot;
mod bot;
mod coin;
mod game_state;

use board::Board;
use game_state::GameState;
use std::fmt;
use std::io::Write;

const CLEAR_SCREEN: &str = "\x1B[2J\x1B[1;1H";

fn main() {
    let mut board = Board::new();
    let mut turn = true;
    let mut error_message = String::new();

    while !board.game_over() {
        if turn {
            println!("{}{}", CLEAR_SCREEN, board);
            let column = input(format!(
                "{}Player's turn\nEnter column(1-7): ",
                error_message,
            ));

            let column_number = match column.parse::<u8>() {
                Ok(i) if i > 0 && i < 8 => i,
                _ => {
                    error_message = String::from("Column must be a number from 1 to 7\n");
                    continue;
                }
            };

            match board.drop((column_number - 1) as usize, true) {
                Ok(_) => {
                    error_message = String::new();
                    turn = !turn;
                }
                Err(_) => {
                    error_message = format!("Cannot place coin in column {}\n", column_number)
                }
            }
        } else {
            board
                .drop(bot::get_computer_move(&board.get_board()), false)
                .unwrap();
            turn = !turn;
        }
    }

    println!("{}{}", CLEAR_SCREEN, board);

    match board.game_state() {
        GameState::RedWon => println!("Player Won"),
        GameState::YellowWon => println!("Computer Won"),
        GameState::Draw => println!("Draw"),
        GameState::OnGoing => unreachable!(),
    }

    std::thread::sleep(std::time::Duration::from_secs(5));
    main();
}

fn input(msg: impl fmt::Display) -> String {
    print!("{msg}");
    std::io::stdout().flush().unwrap();

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    buffer.trim().to_string()
}
