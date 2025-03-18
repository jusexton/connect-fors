use std::{fmt, str::FromStr};

use thiserror::Error;

const TOP: [u8; 7] = [6, 13, 20, 27, 34, 41, 48];
const TOP_BITBOARD: u64 = 0b1000000_1000000_1000000_1000000_1000000_1000000_1000000;

#[derive(Error, Debug, PartialEq)]
pub enum MoveError {
    #[error("Column did not have available space.")]
    FullColumn,

    #[error("Moves can not be made on a concluded game board. ")]
    ConcludedGame,
}

#[derive(Debug, Error)]
#[error("Provided value was not a valid column.")]
pub struct ColumnConversionError;

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

    pub const fn to_index(self) -> usize {
        self.to_u8() as usize - 1
    }

    pub const fn to_u8(self) -> u8 {
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

impl TryFrom<u8> for Column {
    type Error = ColumnConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Column::One),
            2 => Ok(Column::Two),
            3 => Ok(Column::Three),
            4 => Ok(Column::Four),
            5 => Ok(Column::Five),
            6 => Ok(Column::Six),
            7 => Ok(Column::Seven),
            _ => Err(ColumnConversionError),
        }
    }
}

impl FromStr for Column {
    type Err = ColumnConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Column::One),
            "2" => Ok(Column::Two),
            "3" => Ok(Column::Three),
            "4" => Ok(Column::Four),
            "5" => Ok(Column::Five),
            "6" => Ok(Column::Six),
            "7" => Ok(Column::Seven),
            _ => Err(ColumnConversionError),
        }
    }
}

#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub enum Player {
    #[default]
    One,
    Two,
}

impl Player {
    const fn from_move_count(count: u8) -> Self {
        match count & 1 {
            0 => Player::One,
            _ => Player::Two,
        }
    }
}

#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub enum BoardStatus {
    Winner(Player),
    Draw,
    #[default]
    OnGoing,
}

#[derive(Debug, Clone)]
pub struct ConnectFourBoard {
    player_one_bitboard: u64,
    player_two_bitboard: u64,
    move_count: u8,
    heights: [u8; 7],
    history: Vec<Column>,
}

impl ConnectFourBoard {
    pub fn slots(&self) -> BoardSlots {
        BoardSlots::new(self)
    }

    pub fn try_move(&mut self, column: Column) -> Result<u64, MoveError> {
        if self.status() != BoardStatus::OnGoing {
            return Err(MoveError::ConcludedGame);
        }

        if !self.is_playable(column) {
            return Err(MoveError::FullColumn);
        }

        let idx = column.to_index();
        let next_position = 1 << self.heights[idx];
        match self.current_player() {
            Player::One => self.player_one_bitboard ^= next_position,
            Player::Two => self.player_two_bitboard ^= next_position,
        };

        self.move_count += 1;
        self.heights[idx] += 1;
        self.history.push(column);

        Ok(next_position)
    }

    pub fn pop_move(&mut self) {
        if let Some(column) = self.history.pop() {
            let idx = column.to_index();
            self.move_count -= 1;
            self.heights[idx] -= 1;

            let old_position = 1 << self.heights[idx];
            match self.current_player() {
                Player::One => self.player_one_bitboard ^= old_position,
                Player::Two => self.player_two_bitboard ^= old_position,
            };
        }
    }

    pub fn is_playable(&self, column: Column) -> bool {
        let idx = column.to_index();
        self.heights[idx] < TOP[idx]
    }

    pub fn player_one_bitboard(&self) -> u64 {
        self.player_one_bitboard
    }

    pub fn player_two_bitboard(&self) -> u64 {
        self.player_two_bitboard
    }

    pub fn column_height(&self, column: Column) -> u8 {
        let idx = column.to_index();
        self.heights[idx] % 7
    }

    pub fn current_player(&self) -> Player {
        Player::from_move_count(self.move_count)
    }

    pub fn status(&self) -> BoardStatus {
        if has_winner(self.player_one_bitboard) {
            return BoardStatus::Winner(Player::One);
        }
        if has_winner(self.player_two_bitboard) {
            return BoardStatus::Winner(Player::Two);
        }
        if self.move_count == 42 {
            return BoardStatus::Draw;
        }
        BoardStatus::OnGoing
    }
}

impl Default for ConnectFourBoard {
    fn default() -> Self {
        Self {
            player_one_bitboard: 0,
            player_two_bitboard: 0,
            move_count: 0,
            heights: [0, 7, 14, 21, 28, 35, 42],
            history: Vec::with_capacity(42),
        }
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
        Self { cursor: 0, board }
    }
}

impl Iterator for BoardSlots<'_> {
    type Item = Slot;

    fn next(&mut self) -> Option<Self::Item> {
        if (1 << self.cursor) & TOP_BITBOARD != 0 {
            self.cursor += 1;
        }

        if self.cursor >= 48 {
            return None;
        }

        let slot = if self.board.player_one_bitboard() & (1 << self.cursor) != 0 {
            Slot::Occupied(Player::One)
        } else if self.board.player_two_bitboard() & (1 << self.cursor) != 0 {
            Slot::Occupied(Player::Two)
        } else {
            Slot::Vacant
        };
        self.cursor += 1;
        Some(slot)
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{BoardStatus, MoveError, Player, Slot};

    use super::{Column, ConnectFourBoard, has_winner};

    #[test]
    fn default_board() {
        let board = ConnectFourBoard::default();

        assert_eq!(0, board.player_one_bitboard());
        assert_eq!(0, board.player_two_bitboard());
        assert_eq!(Player::One, board.current_player());
        assert_eq!(BoardStatus::OnGoing, board.status());
    }

    #[test]
    fn places_piece_in_column_one() {
        let mut board = ConnectFourBoard::default();
        let next_pos = board.try_move(Column::One).unwrap();
        assert_eq!(
            0b000000_0000000_0000000_0000000_0000000_0000000_0000001,
            next_pos
        );
    }

    #[test]
    fn places_piece_in_column_two() {
        let mut board = ConnectFourBoard::default();
        let next_pos = board.try_move(Column::Two).unwrap();
        assert_eq!(
            0b0000000_0000000_0000000_0000000_0000000_0000001_0000000,
            next_pos
        );
    }

    #[test]
    fn places_piece_in_column_three() {
        let mut board = ConnectFourBoard::default();
        let next_pos = board.try_move(Column::Three).unwrap();
        assert_eq!(
            0b0000000_0000000_0000000_0000000_0000001_0000000_0000000,
            next_pos
        );
    }

    #[test]
    fn error_when_column_is_full() {
        let mut board = ConnectFourBoard::default();
        for _ in 0..6 {
            let _ = board.try_move(Column::One);
        }
        assert_eq!(Err(MoveError::FullColumn), board.try_move(Column::One));
    }

    #[test]
    fn error_moving_on_concluded_board() {
        let mut board = ConnectFourBoard {
            player_one_bitboard: 0b0000000_0000000_0000000_0000000_0000000_0000000_0001111,
            ..Default::default()
        };
        assert_eq!(Err(MoveError::ConcludedGame), board.try_move(Column::One));
    }

    #[test]
    fn players_alternate_between_moves() {
        let mut board = ConnectFourBoard::default();
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
        assert!(!has_winner(
            0b1110110_1110000_0000000_0001000_0000000_1100000_1000000
        ));
    }

    #[test]
    fn reads_board_slots() {
        let board = ConnectFourBoard {
            player_one_bitboard: 0b0100000_0000000_0100000_0000000_0100000_0000000_0111111,
            player_two_bitboard: 0b0000000_0100000_0000000_0100000_0000000_0011111_0000000,
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
