use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use tui_textarea::TextArea;

use crate::model::InputMode;
use crate::operation::get_showing_tasks;
use crate::App;

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Length(3),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App, textarea: &mut TextArea) {
    let (begin, end, highlight_index) = get_showing_tasks(app);
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    // with at least a margin of 1

    let texts: Vec<Spans> = app.tasks[begin..end]
        .iter()
        .enumerate()
        .map(|(index, task)| {
            let mut text_style = Style::default().fg(Color::White).bg(Color::Reset);
            if highlight_index == index {
                text_style = Style::default().fg(Color::Black).bg(Color::White);
            }
            Spans::from(vec![
                Span::raw(format!("{:1$}", "", (task.depth * 4) as usize)),
                Span::raw(format!("{:?} ", task.state)),
                Span::styled(task.content.as_str(), text_style),
            ])
        })
        .collect();

    // Surrounding block
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Task tool designed by alonescar")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    let paragraph = Paragraph::new(texts.clone())
        .block(block)
        .scroll((0, app.scroll));
    f.render_widget(paragraph, app.window_rect);

    match app.input_mode {
        // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
        InputMode::Normal => {}
        InputMode::Editing => {
            let area = centered_rect(60, 12, app.window_rect);
            textarea.set_block(Block::default().borders(Borders::all()));
            f.render_widget(Clear, area); //this clears out the background
            f.render_widget(textarea.widget(), area);
        }
    }
}