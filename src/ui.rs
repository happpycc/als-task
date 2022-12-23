use tui::{
    backend::{Backend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Clear},
    Frame,
};
use tui_textarea::TextArea;

use crate::App;
use crate::model::InputMode;

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
    let size = f.size();
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    // with at least a margin of 1
    let can_showed_num = size.height as usize - 2;
    let tasks_len = app.tasks.len();
    let begin: usize;
    let end: usize;
    let highlight_index: usize;

    // within window
    if can_showed_num >= tasks_len {
        begin = 0;
        end = tasks_len;
        highlight_index = app.index;
    } else {
    // out of window
        if app.index as isize <= can_showed_num as isize / 2 - 1 {
            begin = 0;
            end = can_showed_num;
            highlight_index = app.index
        } else if tasks_len - can_showed_num + can_showed_num / 2 <= app.index {
            begin = tasks_len - can_showed_num;
            end = tasks_len;
            highlight_index = app.index - (tasks_len - can_showed_num);
        }
        else {
            if can_showed_num > 1 {
                if can_showed_num % 2 == 0 {
                    begin = app.index - (can_showed_num / 2 - 1);
                    end = app.index + (can_showed_num / 2) + 1;
                    highlight_index = can_showed_num / 2 - 1;
                } else {
                    begin = app.index - (can_showed_num - 1) / 2;
                    end = app.index + (can_showed_num - 1) / 2 + 1;
                    highlight_index = (can_showed_num + 1) / 2 - 1;
                }
            } else {
                begin = app.index;
                end = app.index + 1;
                highlight_index = 0;
            }
        }
    }

    let texts: Vec<Spans> = 
    app
    .tasks[begin..end]
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
        .block(block);
    f.render_widget(paragraph, size);

    match app.input_mode {
        // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
        InputMode::Normal => {},
        InputMode::Editing => {
            let area = centered_rect(60, 12, size);
            textarea.set_block(
                Block::default()
                    .borders(Borders::all())
            );
            f.render_widget(Clear, area); //this clears out the background
            f.render_widget(textarea.widget(), area);
        }
    }
}