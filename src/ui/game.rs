use std::rc::Rc;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    symbols::Marker,
    text::{Line, Text},
    widgets::{
        Block,
        canvas::{Canvas, Circle},
    },
};

use crate::{
    app::App,
    game::{BoardStatus, ConnectFourBoard, Player, Slot},
};

use super::util;

const SLOT_RADIUS: f64 = 4.0;
const SLOT_PADDING: f64 = 10.0;
const SLOT: f64 = SLOT_RADIUS + SLOT_PADDING;

const BOARD_PADDING: f64 = 15.0;

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = game_page_chunks(frame);

    let title = Line::from(vec![
        "Player One".red(),
        "  VS  ".into(),
        "Player Two".yellow(),
    ]);
    let title_area = util::center(
        chunks[0],
        Constraint::Length(title.width() as u16),
        Constraint::Length(3),
    );
    frame.render_widget(title, title_area);

    let border_color = match app.board().status() {
        BoardStatus::Winner(player) => get_player_color(player),
        BoardStatus::Draw => Color::Gray,
        BoardStatus::OnGoing => Color::White,
    };
    let board_area = board_chunk(chunks[1]);
    let canvas = Canvas::default()
        .block(Block::bordered().fg(border_color))
        .marker(Marker::Braille)
        .paint(|ctx| {
            for slot in build_board_slots(app.board()) {
                ctx.draw(&slot);
            }
        })
        .x_bounds([0.0, SLOT * 7.0 + BOARD_PADDING])
        .y_bounds([0.0, SLOT * 6.0 + BOARD_PADDING]);
    frame.render_widget(canvas, board_area);

    let menu = Text::raw("(1-7) Drop Piece     (q) Exit");
    let menu_area = util::center(
        chunks[2],
        Constraint::Length(menu.width() as u16),
        Constraint::Length(3),
    );
    frame.render_widget(menu, menu_area);
}

fn game_page_chunks(frame: &mut Frame) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(frame.area())
}

fn board_chunk(area: Rect) -> Rect {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Ratio(1, 2),
            Constraint::Fill(1),
        ])
        .split(area);
    chunks[1]
}

fn build_board_slots(board: &ConnectFourBoard) -> Vec<Circle> {
    let mut slots = Vec::with_capacity(42);
    for (idx, slot) in board.slots().enumerate() {
        let color = match slot {
            Slot::Occupied(player) => get_player_color(player),
            Slot::Vacant => Color::White,
        };

        let (row, col) = (idx / 7, idx % 7);
        slots.push(Circle {
            x: (col as f64 * SLOT) + BOARD_PADDING,
            y: (5.0 * 14.0) - (row as f64 * SLOT) + BOARD_PADDING,
            radius: SLOT_RADIUS,
            color,
        });
    }
    slots
}

fn get_player_color(player: Player) -> Color {
    match player {
        Player::One => Color::Red,
        Player::Two => Color::Yellow,
    }
}
