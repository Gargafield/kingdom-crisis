use ratatui::widgets::{Block, BorderType, Borders, Widget};

#[derive(Debug, Default)]
pub struct LogWidget { }

impl Widget for &LogWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let block = Block::default()
            .title(" Log ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        block.render(area, buf);
    }
}