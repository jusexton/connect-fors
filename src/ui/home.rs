use std::rc::Rc;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    text::Text,
};

use super::util;

const TITLE: &str = include_str!("../../assets/title.txt");

const MENU: &str = "
(1) Single Player
(2) Multiplayer
(q) Exit
";

fn prepare_chunks(frame: &Frame) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(7),
            Constraint::Length(4),
            Constraint::Fill(1),
        ])
        .split(frame.area())
}

pub fn draw(frame: &mut Frame) {
    let chunks = prepare_chunks(frame);

    let title = Text::raw(TITLE);
    let title_chunk = util::center(
        chunks[1],
        Constraint::Length(title.width() as u16),
        Constraint::Length(10),
    );
    frame.render_widget(title, title_chunk);

    let menu = Text::raw(MENU);
    let menu_chunk = util::center(
        chunks[2],
        Constraint::Length(menu.width() as u16),
        Constraint::Length(4),
    );
    frame.render_widget(menu, menu_chunk);
}
