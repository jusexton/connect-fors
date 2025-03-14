use rand::seq::IndexedRandom;

use crate::game::{BoardStatus, Column, ConnectFourBoard, Player};

pub fn next_move(board: &ConnectFourBoard, depth: usize) -> Option<Column> {
    match board.status() {
        BoardStatus::OnGoing => {
            let (column, _) = minimax(board, depth, i32::MIN, i32::MAX, false);
            column
        }
        _ => None,
    }
}

fn minimax(
    board: &ConnectFourBoard,
    depth: usize,
    mut alpha: i32,
    mut beta: i32,
    maximizing_player: bool,
) -> (Option<Column>, i32) {
    if depth == 0 || board.status() != BoardStatus::OnGoing {
        return (None, evalulate_board(board));
    }

    if maximizing_player {
        let mut max_eval = i32::MIN;
        let possible_boards = possible_boards(board);
        let mut best_column = possible_boards
            .choose(&mut rand::rng())
            .map(|mv| mv.0)
            .unwrap();
        for (column, possible) in possible_boards {
            let (_, eval) = minimax(&possible, depth - 1, alpha, beta, false);

            if eval > max_eval {
                best_column = column;
                max_eval = eval;
            }

            alpha = alpha.max(eval);
            if beta <= alpha {
                break;
            }
        }
        (Some(best_column), max_eval)
    } else {
        let mut min_eval = i32::MAX;
        let possible_boards = possible_boards(board);
        let mut best_column = possible_boards
            .choose(&mut rand::rng())
            .map(|mv| mv.0)
            .unwrap();
        for (column, possible) in possible_boards {
            let (_, eval) = minimax(&possible, depth - 1, alpha, beta, true);

            if eval < min_eval {
                best_column = column;
                min_eval = eval;
            }

            beta = beta.min(eval);
            if beta <= alpha {
                break;
            }
        }
        (Some(best_column), min_eval)
    }
}

fn possible_boards(board: &ConnectFourBoard) -> Vec<(Column, ConnectFourBoard)> {
    Column::all()
        .into_iter()
        .filter_map(|column| {
            let mut next_board = board.clone();
            match next_board.try_move(column) {
                Ok(_) => Some((column, next_board)),
                Err(_) => None,
            }
        })
        .collect()
}

fn evalulate_board(board: &ConnectFourBoard) -> i32 {
    match board.status() {
        BoardStatus::Winner(player) => match player {
            Player::One => i32::MAX,
            Player::Two => i32::MIN,
        },
        BoardStatus::Draw => 0,
        // TODO: This is basically just a random value and needs to be enhanced
        // in order for the minmax algo to play the game more efficiently.
        BoardStatus::OnGoing => (board.player_one_bitboard() | board.player_two_bitboard()) as i32,
    }
}
