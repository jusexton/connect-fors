use app::App;

mod ai;
mod app;
mod event;
mod game;
mod ui;

fn main() -> anyhow::Result<()> {
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}
