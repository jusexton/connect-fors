use crate::game::{BoardStatus, Column, ConnectFourBoard, Player};

// Each position represents the number of connect 4's that overlap in that position.
#[rustfmt::skip]
const CONNECT_FOUR_MAP: [u8; 49] = [
    3, 4, 5, 5, 4, 3, 0,
    4, 6, 8, 8, 6, 4, 0,
    5, 8, 11, 11, 8, 5, 0,
    7, 9, 13, 13, 9, 7, 0,
    5, 8, 11, 11, 8, 5, 0,
    4, 6, 8, 8, 6, 4, 0,
    3, 4, 5, 5, 4, 3, 0,
];

pub fn next_move(board: &ConnectFourBoard, depth: u8) -> Option<Column> {
    let mut board = board.clone();
    let sign = sign_by_player(board.current_player());
    let mut alpha = -sign * 1000000;
    let beta = -alpha;

    let mut best_move = None;
    for column in Column::all() {
        if !board.is_playable(column) {
            continue;
        }

        let _ = board.try_move(column);
        let score = minimax(&mut board, depth - 1, beta, alpha);
        board.pop_move();

        if sign * score > sign * alpha {
            alpha = score;
            best_move = Some(column);
        }

        if sign * alpha >= sign * beta {
            break;
        }
    }

    best_move
}

fn minimax(board: &mut ConnectFourBoard, depth: u8, mut alpha: i32, beta: i32) -> i32 {
    if depth == 0 || board.status() != BoardStatus::OnGoing {
        return evalulate_board(board);
    }

    let sign = sign_by_player(board.current_player());
    for column in Column::all() {
        if !board.is_playable(column) {
            continue;
        }

        let _ = board.try_move(column);
        let score = minimax(board, depth - 1, beta, alpha);
        board.pop_move();

        if sign * score > sign * alpha {
            alpha = score;
        }
        if sign * alpha >= sign * beta {
            break;
        }
    }

    alpha
}

fn sign_by_player(player: Player) -> i32 {
    match player {
        Player::One => 1,
        Player::Two => -1,
    }
}

fn evalulate_board(board: &ConnectFourBoard) -> i32 {
    match board.status() {
        BoardStatus::Winner(player) => match player {
            Player::One => 100000,
            Player::Two => -100000,
        },
        BoardStatus::Draw => 0,
        BoardStatus::OnGoing => (0..49).fold(0, |mut acc, idx| {
            let possible_score = CONNECT_FOUR_MAP[idx] as i32;
            let player_one = ((board.player_one_bitboard() >> idx) & 1) as i32;
            let player_two = ((board.player_two_bitboard() >> idx) & 1) as i32;
            acc += possible_score * (player_one - player_two);
            acc
        }),
    }
}
