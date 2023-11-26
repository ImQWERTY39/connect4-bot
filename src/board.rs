use std::fmt;

use crate::coin::Coin;
use crate::game_state::GameState;

const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 6;

#[derive(Clone)]
pub struct Board {
    board: Vec<Vec<Coin>>,
    game_state: GameState,
}

impl Board {
    pub fn new() -> Self {
        let mut board_vec = Vec::with_capacity(BOARD_HEIGHT);

        for _ in 0..BOARD_HEIGHT {
            board_vec.push(vec![Coin::Empty; BOARD_WIDTH]);
        }

        Self {
            board: board_vec,
            game_state: GameState::OnGoing,
        }
    }

    pub fn drop(&mut self, col: usize, turn: bool) -> Result<(), ()> {
        let mut row = 0;

        if self.board[row][col] != Coin::Empty {
            return Err(());
        }

        while row + 1 < BOARD_HEIGHT {
            if self.board[row + 1][col] == Coin::Empty {
                row += 1;
            } else {
                break;
            }
        }

        self.board[row][col] = if turn { Coin::Red } else { Coin::Yellow };
        self.update_state();

        Ok(())
    }

    pub fn get_board(&self) -> Vec<Vec<Coin>> {
        self.board.clone()
    }

    pub fn game_state(&self) -> &GameState {
        &self.game_state
    }

    pub fn game_over(&self) -> bool {
        self.game_state != GameState::OnGoing
    }

    fn update_state(&mut self) {
        for i in &self.board {
            for j in i.windows(4) {
                if j[0] == j[1] && j[0] == j[2] && j[0] == j[3] {
                    match j[0] {
                        Coin::Empty => (),
                        Coin::Red => self.game_state = GameState::RedWon,
                        Coin::Yellow => self.game_state = GameState::YellowWon,
                    }
                }
            }
        }

        for i in 0..3 {
            for j in 0..BOARD_WIDTH {
                let top = &self.board[i][j];
                let middle_top = &self.board[i + 1][j];
                let middle_bottom = &self.board[i + 2][j];
                let bottom = &self.board[i + 3][j];

                if top == middle_top && top == middle_bottom && top == bottom {
                    match top {
                        Coin::Empty => (),
                        Coin::Red => self.game_state = GameState::RedWon,
                        Coin::Yellow => self.game_state = GameState::YellowWon,
                    }
                }
            }
        }

        for i in 0..3 {
            for j in 0..4 {
                let top_left = &self.board[i][j];
                let left = &self.board[i + 1][j + 1];
                let right = &self.board[i + 2][j + 2];
                let bottom_right = &self.board[i + 3][j + 3];

                if top_left == left && top_left == right && top_left == bottom_right {
                    match top_left {
                        Coin::Empty => (),
                        Coin::Red => self.game_state = GameState::RedWon,
                        Coin::Yellow => self.game_state = GameState::YellowWon,
                    }
                }
            }
        }

        for i in 0..3 {
            for j in 3..BOARD_WIDTH {
                let top_left = &self.board[i][j];
                let left = &self.board[i + 1][j - 1];
                let right = &self.board[i + 2][j - 2];
                let bottom_right = &self.board[i + 3][j - 3];

                if top_left == left && top_left == right && top_left == bottom_right {
                    match top_left {
                        Coin::Empty => (),
                        Coin::Red => self.game_state = GameState::RedWon,
                        Coin::Yellow => self.game_state = GameState::YellowWon,
                    }
                }
            }
        }

        if self.is_full() && self.game_state == GameState::OnGoing {
            self.game_state = GameState::Draw;
        }
    }

    fn is_full(&self) -> bool {
        for i in &self.board[0] {
            if *i == Coin::Empty {
                return false;
            }
        }

        true
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let numbers = "  1   2   3   4   5   6   7\n";
        let sep_line_top = "┌───┬───┬───┬───┬───┬───┬───┐\n";
        let sep_line = "├───┼───┼───┼───┼───┼───┼───┤\n";
        let sep_line_bottom = "└───┴───┴───┴───┴───┴───┴───┘\n";

        let mut board_str = numbers.to_string() + sep_line_top;

        for i in &self.board {
            for j in i {
                board_str += match j {
                    Coin::Empty => "│   ",
                    Coin::Red => "│ R ",
                    Coin::Yellow => "│ Y ",
                }
            }

            board_str += "│\n";
            board_str += sep_line;
        }

        board_str = board_str[..board_str.len() - sep_line.len()].to_string();
        board_str += sep_line_bottom;

        write!(f, "{board_str}")
    }
}
