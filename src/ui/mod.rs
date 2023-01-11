use tui::{
    backend::Backend,
    Frame, widgets::{Borders, Block, Paragraph, Clear}, text::{Spans, Span}, style::{Style, Color}, layout::Alignment,
};
use tui_textarea::TextArea;

use crate::models::{App, Window, InputMode};

mod layout;
mod tasks;
mod groups;

use self::layout::{make_layout, centered_rect};
use self::groups::make_group_texts;
use self::tasks::make_task_texts;


pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App, textarea: &mut TextArea) {
    let size = f.size();

    // Make groups texts
    let (group_texts, max_groups_len) = make_group_texts(&app, size);

    // Make tasks texts
    let (task_texts, max_tasks_len) = make_task_texts(&app, size);

    // Make Layout
    let chunks = make_layout(size);

    // Draw task_groups ui
    let groups_block = Block::default()
        .title_alignment(Alignment::Center)
        .title(Spans::from(Span::styled("Groups",
            if app.window == Window::Groups {
                Style::default().fg(Color::Black).bg(Color::White)
            } else {Style::default().fg(Color::White).bg(Color::Reset)}
        )))
        .borders(Borders::ALL);

    // Draw tasks ui
    let tasks_block = Block::default()
        .title_alignment(Alignment::Center)
        .title(Spans::from(Span::styled("Tasks",
            if app.window == Window::Tasks {
                Style::default().fg(Color::Black).bg(Color::White)
            } else {Style::default().fg(Color::White).bg(Color::Reset)}
        )))
        .borders(Borders::ALL);

    let group_paragraphs = Paragraph::new(group_texts)
        .block(groups_block);

    let task_paragraphs = Paragraph::new(task_texts)
        .block(tasks_block);

    f.render_widget(group_paragraphs, chunks[0]);
    f.render_widget(task_paragraphs, chunks[1]);

    // If app.InputMode == Insert, then draw input ui
    if app.input_mode == InputMode::Insert {
        let area = centered_rect(60, 12, size);
        textarea.set_block(Block::default().borders(Borders::all()));
        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(textarea.widget(), area);
    }
}


pub fn get_showing_range(len: usize, window_height: u16, index: usize)
    -> (usize, usize, usize)
{
    let begin: usize;
    let end: usize;
    let highlight_index: usize;
    let can_showed_num = window_height as usize - 2;
    // within window
    if can_showed_num >= len {
        begin = 0;
        end = len;
        highlight_index = index;
    } else {
    // out of window
        if index as isize <= can_showed_num as isize / 2 - 1 {
            begin = 0;
            end = can_showed_num;
            highlight_index = index;
        } else if len - can_showed_num + can_showed_num / 2 <= index {
            begin = len - can_showed_num;
            end = len;
            highlight_index = index - (len - can_showed_num);
        }
        else {
            if can_showed_num > 1 {
                if can_showed_num % 2 == 0 {
                    begin = index - (can_showed_num / 2 - 1);
                    end = index + (can_showed_num / 2) + 1;
                    highlight_index = can_showed_num / 2 - 1;
                } else {
                    begin = index - (can_showed_num - 1) / 2;
                    end = index + (can_showed_num - 1) / 2 + 1;
                    highlight_index = (can_showed_num + 1) / 2 - 1;
                }
            } else {
                begin = index;
                end = index + 1;
                highlight_index = 0;
            }
        }
    }
    (begin, end, highlight_index)
}

