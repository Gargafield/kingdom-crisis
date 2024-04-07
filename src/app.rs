use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, symbols::border, widgets::{block::{Position, Title}, Block, BorderType, Borders, Paragraph}};

use crate::widgets::{ActionWidget, GraphWidget, LogWidget, VariablesWidget};

#[derive(Debug, Default)]
pub struct App {
    pub counter: u8,

    action_widget: ActionWidget,
    graph_widget: GraphWidget,
    variables_widget: VariablesWidget,
    log_widget: LogWidget,
}

impl App {
    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Left => self.decrement(),
            KeyCode::Right => self.increment(),
            _ => {}
        }
    }

    fn increment(&mut self) {
        self.counter += 1;
    }

    fn decrement(&mut self) {
        self.counter -= 1;
    }
}

impl App {
    fn prepare_layout(&self, area: Rect) -> [Rect; 5] {
        let layout = Layout::vertical([
            Constraint::Percentage(70),
            Constraint::Percentage(30),
            ]).split(area);

        let top = Layout::horizontal([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3)])
            .areas::<3>(layout[0]);

        let bottom = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(3)])
            .areas::<2>(layout[1]);

        [top[0], top[1], top[2], bottom[0], bottom[1]]
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [actions, graph, variables, idk, log] = self.prepare_layout(area);

        self.action_widget.render(actions, buf);
        self.graph_widget.render(graph, buf);
        self.variables_widget.render(variables, buf);
        self.log_widget.render(log, buf);

        let block = Block::default()
            .title(" IDK ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        block.render(idk, buf);
    }
}
