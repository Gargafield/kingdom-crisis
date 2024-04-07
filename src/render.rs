use ratatui::Frame;

use crate::app::App;

pub fn render_app(app: &App, frame: &mut Frame) {
    frame.render_widget(app, frame.size());
}