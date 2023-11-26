use std::collections::HashMap;

use crate::coin::Coin;

struct ContinuousType {
    diagonal: usize,
    horizontal: usize,
    vertical: usize,
}

#[derive(PartialEq, Eq)]
enum State {
    ComputerWon,
    PlayerWon,
    Draw,
    None,
}

pub fn get_computer_move(board: &Vec<Vec<Coin>>) -> usize {
    get_best_move(board, false)
}

fn get_best_move(board: &Vec<Vec<Coin>>, turn: bool) -> usize {
    let mut move_evaluations = HashMap::new();

    for i in get_legal_moves(board) {
        move_evaluations.insert(i, evaluate_for_move(board, i, turn));
    }

    if turn {
        *move_evaluations
            .iter()
            .max_by(|x, y| x.1.total_cmp(y.1))
            .unwrap()
            .0
    } else {
        *move_evaluations
            .iter()
            .min_by(|x, y| x.1.total_cmp(y.1))
            .unwrap()
            .0
    }
}

fn evaluate_for_move(board: &Vec<Vec<Coin>>, after_move: usize, turn: bool) -> f64 {
    let mut evaluation = 0.0;
    let mut board_copy = board.clone();
    let (comp_choice_row, comp_choice_col) = drop(&mut board_copy, after_move, turn);

    if who_won(&board_copy) == State::ComputerWon {
        return -f64::INFINITY;
    }

    for i in get_legal_moves(board) {
        let (placed_row, placed_col) = drop(&mut board_copy, i as usize, true);

        if who_won(&board_copy) == State::PlayerWon {
            return f64::INFINITY;
        }

        board_copy[placed_row][placed_col] = Coin::Empty;
    }

    let two_count = twos_count(&board_copy, comp_choice_row, comp_choice_col);
    let three_count = threes_count(&board_copy, comp_choice_row, comp_choice_col);

    evaluation += -0.6 * three_count.diagonal as f64;
    evaluation += -0.4 * three_count.horizontal as f64;
    evaluation += -0.2 * three_count.vertical as f64;

    evaluation += -0.3 * two_count.diagonal as f64;
    evaluation += -0.2 * two_count.horizontal as f64;
    evaluation += -0.1 * two_count.vertical as f64;

    get_neighboring_cells(&board_copy, comp_choice_row, comp_choice_col)
        .iter()
        .for_each(|x| {
            evaluation += match x.0 {
                Coin::Empty => 0.0,
                Coin::Red => 0.1,
                Coin::Yellow => -0.1,
            };
        });

    evaluation
}

fn twos_count(board: &Vec<Vec<Coin>>, row: usize, col: usize) -> ContinuousType {
    let coin_type = &board[row][col];
    let mut types = ContinuousType {
        diagonal: 0,
        horizontal: 0,
        vertical: 0,
    };

    let top_left = get_relative_cell(board, row, col, -1, -1).unwrap_or(&Coin::Empty);
    let top_right = get_relative_cell(board, row, col, -1, 1).unwrap_or(&Coin::Empty);
    let left = get_relative_cell(board, row, col, 0, -1).unwrap_or(&Coin::Empty);
    let right = get_relative_cell(board, row, col, 0, 1).unwrap_or(&Coin::Empty);
    let bottom = get_relative_cell(board, row, col, 1, 0).unwrap_or(&Coin::Empty);
    let bottom_left = get_relative_cell(board, row, col, 1, -1).unwrap_or(&Coin::Empty);
    let bottom_right = get_relative_cell(board, row, col, 1, 1).unwrap_or(&Coin::Empty);

    if top_left == coin_type {
        let top_left_2 = get_relative_cell(board, row, col, -2, -2);
        let top_left_3 = get_relative_cell(board, row, col, -3, -3);
        let bottom_right_1 = get_relative_cell(board, row, col, 1, 1);
        let bottom_right_2 = get_relative_cell(board, row, col, 2, 2);

        if let Some(i) = top_left_2 {
            if i.equals_or_empty(coin_type) {
                if let Some(j) = top_left_3 {
                    if j.equals_or_empty(coin_type) {
                        types.diagonal += 1;
                    }
                }

                if let Some(j) = bottom_right_1 {
                    if j.equals_or_empty(coin_type) {
                        types.diagonal += 1;
                    }
                }
            }
        } else if let (Some(i), Some(j)) = (bottom_right_1, bottom_right_2) {
            if i.equals_or_empty(coin_type) && j.equals_or_empty(coin_type) {
                types.diagonal += 1;
            }
        }
    }

    if top_right == coin_type {
        let top_right_2 = get_relative_cell(board, row, col, -2, 2);
        let top_right_3 = get_relative_cell(board, row, col, -3, 3);
        let bottom_left_1 = get_relative_cell(board, row, col, 1, -1);
        let bottom_left_2 = get_relative_cell(board, row, col, 2, -2);

        if let Some(i) = top_right_2 {
            if i.equals_or_empty(coin_type) {
                if let Some(j) = top_right_3 {
                    if j.equals_or_empty(coin_type) {
                        types.diagonal += 1;
                    }
                }

                if let Some(j) = bottom_left_1 {
                    if j.equals_or_empty(coin_type) {
                        types.diagonal += 1;
                    }
                }
            }
        } else if let (Some(i), Some(j)) = (bottom_left_1, bottom_left_2) {
            if i.equals_or_empty(coin_type) && j.equals_or_empty(coin_type) {
                types.diagonal += 1;
            }
        }
    }

    if bottom_left == coin_type {
        let bottom_left_2 = get_relative_cell(board, row, col, 2, -2);
        let bottom_left_3 = get_relative_cell(board, row, col, 3, -3);
        let top_right_1 = get_relative_cell(board, row, col, -1, 1);
        let top_right_2 = get_relative_cell(board, row, col, -2, 2);

        if let Some(i) = bottom_left_2 {
            if i.equals_or_empty(coin_type) {
                if let Some(j) = bottom_left_3 {
                    if j.equals_or_empty(coin_type) {
                        types.diagonal += 1;
                    }
                }

                if let Some(j) = top_right_1 {
                    if j.equals_or_empty(coin_type) {
                        types.diagonal += 1;
                    }
                }
            }
        } else if let (Some(i), Some(j)) = (top_right_1, top_right_2) {
            if i.equals_or_empty(coin_type) && j.equals_or_empty(coin_type) {
                types.diagonal += 1;
            }
        }
    }

    if bottom_right == coin_type {
        let bottom_right_2 = get_relative_cell(board, row, col, 2, 2);
        let bottom_right_3 = get_relative_cell(board, row, col, 3, 3);
        let top_left_1 = get_relative_cell(board, row, col, -1, -1);
        let top_left_2 = get_relative_cell(board, row, col, -2, -2);

        if let Some(i) = bottom_right_2 {
            if i.equals_or_empty(coin_type) {
                if let Some(j) = bottom_right_3 {
                    if j.equals_or_empty(coin_type) {
                        types.diagonal += 1;
                    }
                }

                if let Some(j) = top_left_1 {
                    if j.equals_or_empty(coin_type) {
                        types.diagonal += 1;
                    }
                }
            }
        } else if let (Some(i), Some(j)) = (top_left_1, top_left_2) {
            if i.equals_or_empty(coin_type) && j.equals_or_empty(coin_type) {
                types.diagonal += 1;
            }
        }
    }

    if left == coin_type {
        let left_2 = get_relative_cell(board, row, col, 0, -2);
        let left_3 = get_relative_cell(board, row, col, 0, -3);
        let right_1 = get_relative_cell(board, row, col, 0, 1);
        let right_2 = get_relative_cell(board, row, col, 0, 2);

        if let Some(i) = left_2 {
            if i.equals_or_empty(coin_type) {
                if let Some(j) = left_3 {
                    if j.equals_or_empty(coin_type) {
                        types.horizontal += 1;
                    }
                }

                if let Some(j) = right_1 {
                    if j.equals_or_empty(coin_type) {
                        types.horizontal += 1;
                    }
                }
            }
        } else if let (Some(i), Some(j)) = (right_1, right_2) {
            if i.equals_or_empty(coin_type) && j.equals_or_empty(coin_type) {
                types.horizontal += 1;
            }
        }
    }

    if right == coin_type {
        let right_2 = get_relative_cell(board, row, col, 0, 2);
        let right_3 = get_relative_cell(board, row, col, 0, 3);
        let left_1 = get_relative_cell(board, row, col, 0, -1);
        let left_2 = get_relative_cell(board, row, col, 0, -2);

        if let Some(i) = right_2 {
            if i.equals_or_empty(coin_type) {
                if let Some(j) = right_3 {
                    if j.equals_or_empty(coin_type) {
                        types.horizontal += 1;
                    }
                }

                if let Some(j) = left_1 {
                    if j.equals_or_empty(coin_type) {
                        types.horizontal += 1;
                    }
                }
            }
        } else if let (Some(i), Some(j)) = (left_1, left_2) {
            if i.equals_or_empty(coin_type) && j.equals_or_empty(coin_type) {
                types.horizontal += 1;
            }
        }
    }

    if bottom == coin_type {
        if row > 2 {
            types.vertical += 1;
        }
    }

    types
}

fn threes_count(board: &Vec<Vec<Coin>>, row: usize, col: usize) -> ContinuousType {
    let coin_type = &board[row][col];
    let mut types = ContinuousType {
        diagonal: 0,
        horizontal: 0,
        vertical: 0,
    };

    let top_left_1 = get_relative_cell(board, row, col, -1, -1).unwrap_or(&Coin::Empty);
    let top_left_2 = get_relative_cell(board, row, col, -2, -2).unwrap_or(&Coin::Empty);
    let top_right_1 = get_relative_cell(board, row, col, -1, 1).unwrap_or(&Coin::Empty);
    let top_right_2 = get_relative_cell(board, row, col, -1, 2).unwrap_or(&Coin::Empty);
    let left_1 = get_relative_cell(board, row, col, 0, -1).unwrap_or(&Coin::Empty);
    let left_2 = get_relative_cell(board, row, col, 0, -2).unwrap_or(&Coin::Empty);
    let right_1 = get_relative_cell(board, row, col, 0, 1).unwrap_or(&Coin::Empty);
    let bottom_1 = get_relative_cell(board, row, col, 1, 0).unwrap_or(&Coin::Empty);
    let bottom_2 = get_relative_cell(board, row, col, 2, 0).unwrap_or(&Coin::Empty);
    let bottom_left_1 = get_relative_cell(board, row, col, 1, -1).unwrap_or(&Coin::Empty);
    let bottom_right_1 = get_relative_cell(board, row, col, 1, 1).unwrap_or(&Coin::Empty);

    let right_2 = get_relative_cell(board, row, col, 0, 2);
    let bottom_left_2 = get_relative_cell(board, row, col, 2, -2);
    let bottom_right_2 = get_relative_cell(board, row, col, 1, 2);

    if top_left_1 == coin_type && top_left_2 == coin_type {
        let top_left_3 = get_relative_cell(board, row, col, -3, -3);

        if let Some(i) = top_left_3 {
            if i.equals_or_empty(coin_type) {
                types.diagonal += 1;
            }
        }
    }

    if top_left_1 == coin_type && bottom_right_1 == coin_type {
        if let Some(i) = bottom_right_2 {
            if i.equals_or_empty(coin_type) {
                types.diagonal += 1;
            }
        }
    }

    if top_right_1 == coin_type && top_right_2 == coin_type {
        let top_right_3 = get_relative_cell(board, row, col, -3, 3);

        if let Some(i) = top_right_3 {
            if i.equals_or_empty(coin_type) {
                types.diagonal += 1;
            }
        }
    }

    if top_right_1 == coin_type && bottom_left_1 == coin_type {
        if let Some(i) = bottom_left_2 {
            if i.equals_or_empty(coin_type) {
                types.diagonal += 1;
            }
        }
    }

    if left_1 == coin_type && left_2 == coin_type {
        let left_3 = get_relative_cell(board, row, col, 0, -3);

        if let Some(i) = left_3 {
            if i.equals_or_empty(coin_type) {
                types.horizontal += 1;
            }
        }
    }

    if left_1 == coin_type && right_1 == coin_type {
        if let Some(i) = right_2 {
            if i.equals_or_empty(coin_type) {
                types.horizontal += 1;
            }
        }
    }

    if bottom_1 == coin_type && bottom_2 == coin_type {
        types.vertical += 1;
    }

    types
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

fn get_relative_cell(
    board: &Vec<Vec<Coin>>,
    row: usize,
    col: usize,
    row_shift: isize,
    col_shift: isize,
) -> Option<&Coin> {
    let new_row = (row as isize) + row_shift;
    let new_col = (col as isize) + col_shift;

    if new_row < 0 || new_row > 5 || new_col < 0 || new_col > 6 {
        None
    } else {
        Some(&board[new_row as usize][new_col as usize])
    }
}

fn get_neighboring_cells(
    board: &Vec<Vec<Coin>>,
    row: usize,
    col: usize,
) -> Vec<(&Coin, usize, usize)> {
    let mut neighbors = Vec::new();

    match get_relative_cell(board, row, col, -1, -1) {
        Some(i) => neighbors.push((i, row - 1, col - 1)),
        None => (),
    }

    match get_relative_cell(board, row, col, -1, 1) {
        Some(i) => neighbors.push((i, row - 1, col + 1)),
        None => (),
    }

    match get_relative_cell(board, row, col, 0, -1) {
        Some(i) => neighbors.push((i, row, col - 1)),
        None => (),
    }

    match get_relative_cell(board, row, col, 0, 1) {
        Some(i) => neighbors.push((i, row, col + 1)),
        None => (),
    }

    match get_relative_cell(board, row, col, 1, -1) {
        Some(i) => neighbors.push((i, row, col - 1)),
        None => (),
    }

    match get_relative_cell(board, row, col, 1, 0) {
        Some(i) => neighbors.push((i, row, col)),
        None => (),
    }

    match get_relative_cell(board, row, col, 1, 1) {
        Some(i) => neighbors.push((i, row, col + 1)),
        None => (),
    }

    neighbors
}

fn get_legal_moves(board: &Vec<Vec<Coin>>) -> Vec<usize> {
    let mut moves = Vec::new();

    for i in 0..7 {
        if board[0][i] == Coin::Empty {
            moves.push(i);
        }
    }

    moves
}

fn is_full(board: &Vec<Vec<Coin>>) -> bool {
    for i in board.iter().next().unwrap() {
        if *i == Coin::Empty {
            return false;
        }
    }

    true
}

fn who_won(board: &Vec<Vec<Coin>>) -> State {
    for i in board {
        for j in i.windows(4) {
            if j[0] == j[1] && j[0] == j[2] && j[0] == j[3] {
                match j[0] {
                    Coin::Red => return State::PlayerWon,
                    Coin::Yellow => return State::ComputerWon,
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
                    Coin::Red => return State::PlayerWon,
                    Coin::Yellow => return State::ComputerWon,
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
                    Coin::Red => return State::PlayerWon,
                    Coin::Yellow => return State::ComputerWon,
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
                    Coin::Red => return State::PlayerWon,
                    Coin::Yellow => return State::ComputerWon,
                    _ => (),
                }
            }
        }
    }

    if is_full(board) {
        return State::Draw;
    }

    State::None
}
