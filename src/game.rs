use std::{fmt, str::FromStr};

use thiserror::Error;

const TOP: u64 = 0b0000001_0000001_0000001_0000001_0000001_0000001_0000001;

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
        let offset = 7 * (column.to_u64() - 1);
        let mut mask = 0b1000000_0000000_0000000_0000000_0000000_0000000_0000000u64 >> offset;
        let board = !(self.player_one_bitboard | self.player_two_bitboard);
        for _ in 0..6 {
            if board & mask != 0 {
                return Ok(mask);
            }
            mask >>= 1;
        }
        Err(MoveError::FullColumn)
    }

    fn calculate_board_status(&self) -> BoardStatus {
        if has_winner(self.player_one_bitboard) {
            return BoardStatus::Winner(Player::One);
        }
        if has_winner(self.player_two_bitboard) {
            return BoardStatus::Winner(Player::Two);
        }
        if (self.player_one_bitboard | self.player_two_bitboard).count_ones() == 42 {
            return BoardStatus::Draw;
        }
        BoardStatus::OnGoing
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

fn has_winner(bitboard: u64) -> bool {
    let horizontal = bitboard & (bitboard >> 7);
    let vertical = bitboard & (bitboard >> 1);
    let diag_one = bitboard & (bitboard >> 6);
    let diag_two = bitboard & (bitboard >> 8);
    (diag_one & (diag_one >> (2 * 6)))
        | (horizontal & (horizontal >> (2 * 7)))
        | (diag_two & (diag_two >> (2 * 8)))
        | (vertical & (vertical >> 2))
        != 0
}

#[derive(Debug, PartialEq)]
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
            cursor: 0b1000000_0000000_0000000_0000000_0000000_0000000_0000000,
            board,
        }
    }
}

impl Iterator for BoardSlots<'_> {
    type Item = Slot;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor & TOP != 0 {
            self.cursor >>= 1;
        }

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
    use crate::game::{BoardStatus, MoveError, Player, Slot};

    use super::{Column, ConnectFourBoard, has_winner};

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
        for i in 0..6 {
            let next_pos = board.try_move(Column::One).unwrap();
            assert_eq!(
                0b1000000_0000000_0000000_0000000_0000000_0000000_0000000 >> i,
                next_pos
            );
        }
    }

    #[test]
    fn error_when_column_is_full() {
        let mut board = ConnectFourBoard {
            player_one_bitboard: 0b1111110_0000000_0000000_0000000_0000000_0000000_0000000,
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

    #[test]
    fn determines_when_winner_exists() {
        assert!(has_winner(
            0b1111000_0000000_0000000_0000000_0000000_0000000_0000000
        ));
        assert!(has_winner(
            0b1000000_1000000_1000000_1000000_0000000_0000000_0000000
        ));
        assert!(has_winner(
            0b1000000_0100000_0010000_0001000_0000000_0000000_0000000
        ));
        assert!(has_winner(
            0b0001000_0010000_0100000_1000000_0000000_0000000_0000000
        ));
    }

    #[test]
    fn determines_when_no_winner_exists() {
        assert!(!has_winner(
            0b1110000_0000000_0000000_0000000_0000000_0000000_0000000
        ));
    }

    #[test]
    fn reads_board_slots() {
        let board = ConnectFourBoard {
            player_one_bitboard: 0b1111110_0000000_0000010_0000000_0000010_0000000_0000010,
            player_two_bitboard: 0b0000000_1111100_0000000_0000010_0000000_0000010_0000000,
            ..Default::default()
        };

        let slots: Vec<_> = board.slots().collect();

        let expected_slots = vec![
            Slot::Occupied(Player::One),
            Slot::Occupied(Player::One),
            Slot::Occupied(Player::One),
            Slot::Occupied(Player::One),
            Slot::Occupied(Player::One),
            Slot::Occupied(Player::One),
            Slot::Occupied(Player::Two),
            Slot::Occupied(Player::Two),
            Slot::Occupied(Player::Two),
            Slot::Occupied(Player::Two),
            Slot::Occupied(Player::Two),
            Slot::Vacant,
            Slot::Vacant,
            Slot::Vacant,
            Slot::Vacant,
            Slot::Vacant,
            Slot::Vacant,
            Slot::Occupied(Player::One),
            Slot::Vacant,
            Slot::Vacant,
            Slot::Vacant,
            Slot::Vacant,
            Slot::Vacant,
            Slot::Occupied(Player::Two),
            Slot::Vacant,
            Slot::Vacant,
            Slot::Vacant,
            Slot::Vacant,
            Slot::Vacant,
            Slot::Occupied(Player::One),
            Slot::Vacant,
            Slot::Vacant,
            Slot::Vacant,
            Slot::Vacant,
            Slot::Vacant,
            Slot::Occupied(Player::Two),
            Slot::Vacant,
            Slot::Vacant,
            Slot::Vacant,
            Slot::Vacant,
            Slot::Vacant,
            Slot::Occupied(Player::One),
        ];
        assert_eq!(expected_slots, slots);
    }
}
