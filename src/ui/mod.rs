use ratatui::Frame;

use crate::app::App;

mod game;
mod home;
mod util;

pub fn draw_home(frame: &mut Frame) {
    home::draw(frame);
}

pub fn draw_game(frame: &mut Frame, app: &App) {
    game::draw(frame, app);
}
