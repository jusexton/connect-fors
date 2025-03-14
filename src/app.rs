use std::str::FromStr;

use ratatui::{DefaultTerminal, Frame};

use crate::{
    ai,
    event::{Key, TerminalEvent, TerminalEvents},
    game::{BoardStatus, Column, ConnectFourBoard},
    ui,
};

pub enum Page {
    Home,
    SinglePlayer,
    MultiPlayer,
}

pub struct App {
    running: bool,
    current_page: Page,
    board: ConnectFourBoard,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            current_page: Page::Home,
            board: ConnectFourBoard::default(),
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
            Page::SinglePlayer => self.handle_singeplayer_key_press(key),
            Page::MultiPlayer => self.handle_multiplayer_key_press(key),
        }
    }

    fn handle_home_key_press(&mut self, key: Key) {
        match key {
            Key::Char('1') => {
                self.board = ConnectFourBoard::default();
                self.current_page = Page::SinglePlayer;
            }
            Key::Char('2') => {
                self.board = ConnectFourBoard::default();
                self.current_page = Page::MultiPlayer;
            }
            Key::Char('q') => self.running = false,
            _ => {}
        }
    }

    fn handle_singeplayer_key_press(&mut self, key: Key) {
        match key {
            Key::Char('q') => {
                self.current_page = Page::Home;
            }
            Key::Char(c) => {
                if self.board().status() == BoardStatus::OnGoing {
                    if let Ok(column) = Column::from_str(&c.to_string()) {
                        if self.board.try_move(column).is_ok() {
                            if let Some(mv) = ai::next_move(&self.board, 8) {
                                let _ = self.board.try_move(mv);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_multiplayer_key_press(&mut self, key: Key) {
        match key {
            Key::Char('q') => {
                self.current_page = Page::Home;
            }
            Key::Char(c) => {
                if self.board().status() == BoardStatus::OnGoing {
                    if let Ok(column) = Column::from_str(&c.to_string()) {
                        let _ = self.board.try_move(column);
                    }
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
}
