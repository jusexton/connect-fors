use std::{fmt, str::FromStr};

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum MoveError {
    #[error("Column did not have available space.")]
    FullColumn,

    #[error("Moves can not be made on a concluded game board. ")]
    ConcludedGame,
}

#[derive(Debug, Error)]
#[error("Provided value was not a valid column.")]
pub struct ColumnParseError;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Column {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Column {
    pub const fn all() -> [Column; 7] {
        [
            Column::One,
            Column::Two,
            Column::Three,
            Column::Four,
            Column::Five,
            Column::Six,
            Column::Seven,
        ]
    }

    pub const fn to_u64(self) -> u64 {
        match self {
            Column::One => 1,
            Column::Two => 2,
            Column::Three => 3,
            Column::Four => 4,
            Column::Five => 5,
            Column::Six => 6,
            Column::Seven => 7,
        }
    }
}

impl FromStr for Column {
    type Err = ColumnParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Column::One),
            "2" => Ok(Column::Two),
            "3" => Ok(Column::Three),
            "4" => Ok(Column::Four),
            "5" => Ok(Column::Five),
            "6" => Ok(Column::Six),
            "7" => Ok(Column::Seven),
            _ => Err(ColumnParseError),
        }
    }
}

#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub enum Player {
    #[default]
    One,
    Two,
}

#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub enum BoardStatus {
    Winner(Player),
    Draw,
    #[default]
    OnGoing,
}

#[derive(Default, Clone)]
pub struct ConnectFourBoard {
    player_one_bitboard: u64,
    player_two_bitboard: u64,
    turn: Player,
    status: BoardStatus,
}

impl ConnectFourBoard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn slots(&self) -> BoardSlots {
        BoardSlots::new(self)
    }

    pub fn try_move(&mut self, column: Column) -> Result<u64, MoveError> {
        if self.status != BoardStatus::OnGoing {
            return Err(MoveError::ConcludedGame);
        }

        let next_position = self.next_position(column)?;
        match self.turn {
            Player::One => self.player_one_bitboard ^= next_position,
            Player::Two => self.player_two_bitboard ^= next_position,
        };
        self.turn = self.next_turn();
        self.status = self.calculate_board_status();

        Ok(next_position)
    }

    fn next_position(&self, column: Column) -> Result<u64, MoveError> {
        let column_mask = 0b100000010000001000000100000010000001000000u64 >> (column.to_u64() - 1);
        let free_positions = !(self.player_one_bitboard | self.player_two_bitboard) & column_mask;

        match free_positions != 0 {
            true => Ok(free_positions & (!free_positions + 1)),
            false => Err(MoveError::FullColumn),
        }
    }

    fn calculate_board_status(&self) -> BoardStatus {
        if ConnectFourBoard::has_winner(self.player_one_bitboard) {
            return BoardStatus::Winner(Player::One);
        }
        if ConnectFourBoard::has_winner(self.player_two_bitboard) {
            return BoardStatus::Winner(Player::Two);
        }
        if (self.player_one_bitboard | self.player_two_bitboard).count_ones() == 42 {
            return BoardStatus::Draw;
        }
        BoardStatus::OnGoing
    }

    fn has_winner(bitboard: u64) -> bool {
        // TODO: Bug here. This current logic sometime produces the wrong results.
        let edge_mask = 0b1111110_1111110_1111110_1111110_1111110_1111111;
        let safe_board = bitboard & edge_mask;

        let horizontal = safe_board & (safe_board >> 1) & (safe_board >> 2) & (safe_board >> 3);
        let vertical = bitboard & (bitboard >> 7) & (bitboard >> 14) & (bitboard >> 21);
        let diag_up = bitboard & (bitboard >> 6) & (bitboard >> 12) & (bitboard >> 18);
        let diag_down = bitboard & (bitboard >> 8) & (bitboard >> 16) & (bitboard >> 24);

        // Found at: https://stackoverflow.com/questions/7033165/algorithm-to-check-a-connect-four-field
        // Will attempt to use this once the initial UI overhaul is complete.
        // let y = bitboard & (bitboard >> 6);
        // let horizontal = y & (y >> (2 * 7));
        // let vertical = y & (y >> 2);
        // let diag_down = y & (y >> (2 * 6));
        // let diag_up = y & (y >> (2 * 8));

        horizontal != 0 || vertical != 0 || diag_up != 0 || diag_down != 0
    }

    pub fn next_turn(&self) -> Player {
        match self.turn {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }

    pub fn player_one_bitboard(&self) -> u64 {
        self.player_one_bitboard
    }

    pub fn player_two_bitboard(&self) -> u64 {
        self.player_two_bitboard
    }

    pub fn turn(&self) -> Player {
        self.turn
    }

    pub fn status(&self) -> BoardStatus {
        self.status
    }
}

pub enum Slot {
    Occupied(Player),
    Vacant,
}

pub struct BoardSlots<'a> {
    cursor: u64,
    board: &'a ConnectFourBoard,
}

impl<'a> BoardSlots<'a> {
    pub fn new(board: &'a ConnectFourBoard) -> Self {
        Self {
            cursor: 0b100000000000000000000000000000000000000000,
            board,
        }
    }
}

impl Iterator for BoardSlots<'_> {
    type Item = Slot;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cursor {
            0 => None,
            _ => {
                let slot = if self.board.player_one_bitboard() & self.cursor != 0 {
                    Slot::Occupied(Player::One)
                } else if self.board.player_two_bitboard() & self.cursor != 0 {
                    Slot::Occupied(Player::Two)
                } else {
                    Slot::Vacant
                };
                self.cursor >>= 1;
                Some(slot)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{BoardStatus, MoveError, Player};

    use super::{Column, ConnectFourBoard};

    #[test]
    fn default_board() {
        let board = ConnectFourBoard::new();

        assert_eq!(0, board.player_one_bitboard());
        assert_eq!(0, board.player_two_bitboard());
        assert_eq!(Player::One, board.turn());
        assert_eq!(BoardStatus::OnGoing, board.status());
    }

    #[test]
    fn places_new_pieces_in_correct_position() {
        let mut board = ConnectFourBoard::new();
        for i in 0..5 {
            let next_pos = board.try_move(Column::One).unwrap();
            assert_eq!(
                0b000000000000000000000000000000000001000000 << (7 * i),
                next_pos
            );
        }
    }

    #[test]
    fn error_when_column_is_full() {
        let mut board = ConnectFourBoard {
            player_one_bitboard: 0b100000010000001000000100000010000001000000,
            ..Default::default()
        };
        assert_eq!(Err(MoveError::FullColumn), board.try_move(Column::One));
    }

    #[test]
    fn error_moving_on_concluded_board() {
        let mut board = ConnectFourBoard {
            status: BoardStatus::Draw,
            ..Default::default()
        };
        assert_eq!(Err(MoveError::ConcludedGame), board.try_move(Column::One));
    }

    #[test]
    fn players_alternate_between_moves() {
        let mut board = ConnectFourBoard::new();
        assert_eq!(0, board.player_one_bitboard());
        assert_eq!(0, board.player_two_bitboard());

        let _ = board.try_move(Column::One);
        assert_ne!(0, board.player_one_bitboard());
        assert_eq!(0, board.player_two_bitboard());

        let _ = board.try_move(Column::One);
        assert_ne!(0, board.player_one_bitboard());
        assert_ne!(0, board.player_two_bitboard());
    }

    #[test]
    fn next_player_turn_calculated() {
        let board = ConnectFourBoard::new();
        assert_eq!(Player::One, board.turn());
        assert_eq!(Player::Two, board.next_turn());
    }
}
