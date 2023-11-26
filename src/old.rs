use std::collections::HashMap;

use crate::coin::Coin;

pub fn get_computer_move(board: Vec<Vec<Coin>>) -> u8 {
    get_best_move(&board).0
}

fn get_best_move(board: &Vec<Vec<Coin>>) -> (u8, f64) {
    let mut moves_rating = HashMap::<u8, f64>::new();

    for i in get_legal_moves(board) {
        moves_rating.insert(i, evaluate_for_move(board, i, false));
    }

    let (col, eval) = moves_rating
        .iter()
        .min_by(|x, y| x.1.total_cmp(y.1))
        .unwrap();

    (*col, *eval)
}

fn evaluate_for_move(board: &Vec<Vec<Coin>>, after_move: u8, turn: bool) -> f64 {
    let mut board_copy = board.clone();
    let (comp_choice_row, comp_choice_col) = drop(&mut board_copy, after_move as usize, turn);

    let mut count = 0.0;

    if computer_won(&board_copy) {
        return -f64::INFINITY;
    }

    let opponent_moves = get_legal_moves(board);

    for i in opponent_moves {
        let (placed_row, placed_col) = drop(&mut board_copy, i as usize, true);

        if player_won(&board_copy) {
            return f64::INFINITY;
        }

        board_copy[placed_row][placed_col] = Coin::Empty;
    }

    if is_full(board) {
        return 0.0;
    }

    get_neighboring_cells(&board_copy, comp_choice_row, comp_choice_col)
        .iter()
        .for_each(|x| {
            count += match x.0 {
                Coin::Empty => 0.0,
                Coin::Red => 0.5,
                Coin::Yellow => -0.5,
            };
        });

    count
}

fn get_legal_moves(board: &Vec<Vec<Coin>>) -> Vec<u8> {
    let mut moves = Vec::new();

    for i in 0..7 {
        if board[0][i] == Coin::Empty {
            moves.push(i as u8);
        }
    }

    moves
}

fn drop(board: &mut Vec<Vec<Coin>>, col: usize, turn: bool) -> (usize, usize) {
    let mut row = 0;

    while row + 1 < 6 {
        if board[row + 1][col] == Coin::Empty {
            row += 1;
        } else {
            break;
        }
    }

    board[row][col] = if turn { Coin::Red } else { Coin::Yellow };
    (row, col)
}

fn get_neighboring_cells(
    board: &Vec<Vec<Coin>>,
    row: usize,
    col: usize,
) -> Vec<(&Coin, usize, usize)> {
    let mut neighbors = Vec::new();

    if (row as isize) - 1 >= 0 && (col as isize) - 1 >= 0 {
        neighbors.push((&board[row - 1][col - 1], row - 1, col - 1));
    }

    if (row as isize) - 1 >= 0 && (col as isize) + 1 < 7 {
        neighbors.push((&board[row - 1][col + 1], row - 1, col + 1));
    }

    if (col as isize) - 1 >= 0 {
        neighbors.push((&board[row][col - 1], row, col - 1));
    }

    if (col as isize) + 1 < 7 {
        neighbors.push((&board[row][col + 1], row, col + 1));
    }

    if (row as isize) + 1 < 6 && (col as isize) - 1 >= 0 {
        neighbors.push((&board[row + 1][col - 1], row + 1, col - 1));
    }

    if (row as isize) + 1 < 6 {
        neighbors.push((&board[row + 1][col], row + 1, col));
    }

    if (row as isize) + 1 < 6 && (col as isize) + 1 < 7 {
        neighbors.push((&board[row + 1][col + 1], row + 1, col + 1));
    }

    neighbors
}

fn player_won(board: &Vec<Vec<Coin>>) -> bool {
    for i in board {
        for j in i.windows(4) {
            if j[0] == j[1] && j[0] == j[2] && j[0] == j[3] {
                match j[0] {
                    Coin::Red => return true,
                    _ => (),
                }
            }
        }
    }

    for i in 0..3 {
        for j in 0..6 {
            let top = &board[i][j];
            let middle_top = &board[i + 1][j];
            let middle_bottom = &board[i + 2][j];
            let bottom = &board[i + 3][j];

            if top == middle_top && top == middle_bottom && top == bottom {
                match top {
                    Coin::Red => return true,
                    _ => (),
                }
            }
        }
    }

    for i in 0..3 {
        for j in 0..4 {
            let top_left = &board[i][j];
            let left = &board[i + 1][j + 1];
            let right = &board[i + 2][j + 2];
            let bottom_right = &board[i + 3][j + 3];

            if top_left == left && top_left == right && top_left == bottom_right {
                match top_left {
                    Coin::Red => return true,
                    _ => (),
                }
            }
        }
    }

    for i in 0..3 {
        for j in 3..7 {
            let top_right = &board[i][j];
            let right = &board[i + 1][j - 1];
            let left = &board[i + 2][j - 2];
            let bottom_left = &board[i + 3][j - 3];

            if top_right == right && top_right == left && top_right == bottom_left {
                match top_right {
                    Coin::Red => return true,
                    _ => (),
                }
            }
        }
    }

    false
}

fn computer_won(board: &Vec<Vec<Coin>>) -> bool {
    for i in board {
        for j in i.windows(4) {
            if j[0] == j[1] && j[0] == j[2] && j[0] == j[3] {
                match j[0] {
                    Coin::Yellow => return true,
                    _ => (),
                }
            }
        }
    }

    for i in 0..3 {
        for j in 0..6 {
            let top = &board[i][j];
            let middle_top = &board[i + 1][j];
            let middle_bottom = &board[i + 2][j];
            let bottom = &board[i + 3][j];

            if top == middle_top && top == middle_bottom && top == bottom {
                match top {
                    Coin::Yellow => return true,
                    _ => (),
                }
            }
        }
    }

    for i in 0..3 {
        for j in 0..4 {
            let top_left = &board[i][j];
            let left = &board[i + 1][j + 1];
            let right = &board[i + 2][j + 2];
            let bottom_right = &board[i + 3][j + 3];

            if top_left == left && top_left == right && top_left == bottom_right {
                match top_left {
                    Coin::Yellow => return true,
                    _ => (),
                }
            }
        }
    }

    for i in 0..3 {
        for j in 3..7 {
            let top_right = &board[i][j];
            let right = &board[i + 1][j - 1];
            let left = &board[i + 2][j - 2];
            let bottom_left = &board[i + 3][j - 3];

            if top_right == right && top_right == left && top_right == bottom_left {
                match top_right {
                    Coin::Yellow => return true,
                    _ => (),
                }
            }
        }
    }

    false
}

fn is_full(board: &Vec<Vec<Coin>>) -> bool {
    for i in board.iter().next().unwrap() {
        if *i == Coin::Empty {
            return false;
        }
    }

    true
}
