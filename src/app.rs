use core::panic;

use ratatui::{DefaultTerminal, Frame};

use crate::{
    ai,
    event::{Key, TerminalEvent, TerminalEvents},
    game::{BoardStatus, Column, ConnectFourBoard},
    ui,
};

pub enum CursorMovement {
    Left,
    Right,
    Exact(Column),
}

pub enum Page {
    Home,
    SinglePlayer,
    MultiPlayer,
}

pub struct App {
    running: bool,
    current_page: Page,
    board: ConnectFourBoard,
    board_cursor: Option<Column>,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            current_page: Page::Home,
            board: ConnectFourBoard::default(),
            board_cursor: None,
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> anyhow::Result<()> {
        let events = TerminalEvents::listen();
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            if let TerminalEvent::Input(key) = events.next()? {
                self.handle_key_press(key)
            }
        }
        Ok(())
    }

    fn handle_key_press(&mut self, key: Key) {
        match self.current_page {
            Page::Home => self.handle_home_key_press(key),
            Page::SinglePlayer => self.handle_singleplayer_key_press(key),
            Page::MultiPlayer => self.handle_multiplayer_key_press(key),
        }
    }

    fn handle_home_key_press(&mut self, key: Key) {
        match key {
            Key::Char('1') => {
                self.board = ConnectFourBoard::default();
                self.board_cursor = Some(Column::Four);
                self.current_page = Page::SinglePlayer;
            }
            Key::Char('2') => {
                self.board = ConnectFourBoard::default();
                self.board_cursor = Some(Column::Four);
                self.current_page = Page::MultiPlayer;
            }
            Key::Char('q') => self.running = false,
            _ => {}
        }
    }

    fn handle_game_key_press<F>(&mut self, key: Key, mut on_turn: F)
    where
        F: FnMut(&mut ConnectFourBoard, Column),
    {
        match key {
            Key::Char('q') => {
                self.current_page = Page::Home;
            }
            Key::Left => self.update_cursor(CursorMovement::Left),
            Key::Right => self.update_cursor(CursorMovement::Right),
            Key::Char('1') => self.update_cursor(CursorMovement::Exact(Column::One)),
            Key::Char('2') => self.update_cursor(CursorMovement::Exact(Column::Two)),
            Key::Char('3') => self.update_cursor(CursorMovement::Exact(Column::Three)),
            Key::Char('4') => self.update_cursor(CursorMovement::Exact(Column::Four)),
            Key::Char('5') => self.update_cursor(CursorMovement::Exact(Column::Five)),
            Key::Char('6') => self.update_cursor(CursorMovement::Exact(Column::Six)),
            Key::Char('7') => self.update_cursor(CursorMovement::Exact(Column::Seven)),
            Key::Enter if self.board().status() == BoardStatus::OnGoing => {
                on_turn(&mut self.board, self.board_cursor.unwrap())
            }
            _ => {}
        }
    }

    fn handle_singleplayer_key_press(&mut self, key: Key) {
        self.handle_game_key_press(key, |board, cursor| {
            if board.try_move(cursor).is_ok() {
                if let Some(mv) = ai::next_move(board, 10) {
                    let _ = board.try_move(mv);
                } else {
                    panic!("AI was not able to find a move.")
                }
            }
        });
    }

    fn handle_multiplayer_key_press(&mut self, key: Key) {
        self.handle_game_key_press(key, |board, cursor| {
            let _ = board.try_move(cursor);
        });
    }

    fn update_cursor(&mut self, cursor: CursorMovement) {
        match cursor {
            CursorMovement::Left if self.board_cursor.is_some() => {
                let col = self.board_cursor.unwrap().to_u8();
                if let Ok(column) = Column::try_from(col - 1) {
                    self.board_cursor = Some(column);
                }
            }
            CursorMovement::Right if self.board_cursor.is_some() => {
                let col = self.board_cursor.unwrap().to_u8();
                if let Ok(column) = Column::try_from(col + 1) {
                    if self.board.is_playable(column) {
                        self.board_cursor = Some(column);
                    }
                }
            }
            CursorMovement::Exact(column) => {
                if self.board.is_playable(column) {
                    self.board_cursor = Some(column);
                }
            }
            _ => {}
        }
    }

    fn draw(&self, frame: &mut Frame) {
        match self.current_page {
            Page::Home => ui::draw_home(frame),
            Page::SinglePlayer | Page::MultiPlayer => ui::draw_game(frame, self),
        }
    }

    pub fn board(&self) -> &ConnectFourBoard {
        &self.board
    }

    pub fn board_cursor(&self) -> Option<Column> {
        self.board_cursor
    }
}
