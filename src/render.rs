use ratatui::{layout::{Constraint, Layout, Rect}, style::{self, Modifier, Style, Stylize}, symbols::Marker, text::Text, widgets::{Axis, Block, BorderType, Borders, Chart, Dataset, GraphType, List, ListItem, ListState}, Frame};

use crate::{app::App, variable::Variable};

pub fn render_app(app: &App, frame: &mut Frame) {
    let [variables, actions, graph, log] = prepare_layout(frame.size());

    render_variables(frame, app, variables);
    render_action(frame, app, actions);
    render_graph(frame, app, graph);
    render_log(frame, app, log);
}

fn prepare_layout(area: Rect) -> [Rect; 4] {
    let layout = Layout::horizontal([
        Constraint::Percentage(30),
        Constraint::Percentage(70),
        ]).split(area);

    let left = Layout::vertical([
        Constraint::Fill(2),
        Constraint::Fill(1)])
        .areas::<2>(layout[0]);

    let right = Layout::vertical([
        Constraint::Fill(2),
        Constraint::Fill(1)])
        .areas::<2>(layout[1]);

    [left[0], left[1], right[0], right[1]]
}

fn render_variables(frame: &mut Frame, app: &App, area: Rect) {
    let mut items: Vec<ListItem> = Vec::new();
    for (index, variable) in app.variables.iter().enumerate() {
        let change = variable.change();

        let text = format!(
            "{} {} {:.2} {}",
            variable.display.emoji,
            variable.display.name,
            variable.value,
            if change > 0.0 {
                '↑'
            } else if change < 0.0 {
                '↓'
            } else {
                ' '
            },
        );
        
        let text_color = if change > 0.0 {
            style::Color::Green
        } else if change < 0.0 {
            style::Color::Red
        } else {
            style::Color::White
        };

        let item = ListItem::new(Text::styled(
            text,
            style::Style::default().fg(text_color)
        ));

        if index == app.hover_index {
            items.push(item.blue().style(Style::default().add_modifier(Modifier::REVERSED))); 
        }
        else {
            items.push(item);
        }
    }

    let block = Block::default()
        .title(" Variables ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let list = List::new(items)
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .block(block);

    frame.render_stateful_widget(
        list,
        area,
        &mut ListState::default().with_selected(Some(app.selected_variable as usize)),
    );
}

fn render_action(frame: &mut Frame, app: &App, area: Rect) {
    let mut items: Vec<ListItem> = Vec::new();
    for (index, action) in app.actions.iter().enumerate() {
        let text = format!(
            "{} {}",
            action.display.emoji,
            action.display.name,
        );
        
        let item = ListItem::new(Text::styled(
            text,
            style::Style::default().fg(style::Color::White)
            ));

        if app.hover_index > (Variable::COUNT - 1) && index == (app.hover_index - Variable::COUNT) {
            items.push(item.blue().style(Style::default().add_modifier(Modifier::REVERSED))); 
        }
        else {
            items.push(item);
        }
    }

    let block = Block::default()
        .title(" Actions ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let list = List::new(items)
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .block(block);

    frame.render_stateful_widget(
        list,
        area,
        &mut ListState::default().with_selected(Some(app.selected_action as usize)),
    );
}

fn render_graph(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" Graph ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let variable = app.get_variable(&app.selected_variable);
    let data = variable.last_values.iter().enumerate().map(|(i, &v)| (i as f64, v));

    let (min_value, max_value) = variable.last_values.iter()
        .fold((f64::MAX, f64::MIN), |(min, max), &v| (min.min(v), max.max(v)));

    let binding = data.collect::<Vec<_>>();
    let dataset = Dataset::default()
        .marker(Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().magenta())
        .data(&binding);

    let x_axis = Axis::default()
        .title("Time")
        .style(Style::default().hidden())
        .bounds([0.0, variable.max_length as f64]);

    let y_axis = Axis::default()
        .title(variable.display.name)
        .bounds([min_value, max_value]);
    
    let chart = Chart::new(vec![dataset])
        .block(block)
        .x_axis(x_axis)
        .y_axis(y_axis);

    frame.render_widget(chart, area);
}

fn render_log(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" Log ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let items = app
        .log
        .iter()
        .map(|message| ListItem::new(Text::raw(message.to_string())))
        .collect::<Vec<_>>();

    let log = List::new(items)
        .block(block)
        .style(Style::default().fg(style::Color::White));

    frame.render_widget(log, area);
}
