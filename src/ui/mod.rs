use tui::{
    backend::Backend,
    layout::Alignment,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};
use tui_textarea::TextArea;

use crate::models::{App, InputMode, InsertPosistion, Window};

mod groups;
mod layout;
mod tasks;

use self::groups::make_group_texts;
use self::layout::{centered_rect, make_layout};
use self::tasks::make_task_texts;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App, textarea: &mut TextArea) {
    let size = f.size();

    // Make Layout
    let chunks = make_layout(size);

    // task_groups
    // Make groups texts
    let (group_texts, groups_y_max): (Vec<Spans>, i16) = make_group_texts(&app, chunks[0]);

    // Draw task_groups ui
    let groups_block = Block::default()
        .title_alignment(Alignment::Center)
        .title(Spans::from(Span::styled(
            "Groups",
            if app.window == Window::Groups {
                Style::default().fg(Color::Black).bg(Color::White)
            } else {
                Style::default().fg(Color::White).bg(Color::Reset)
            },
        )))
        .borders(Borders::ALL);

    let group_paragraphs = Paragraph::new(group_texts)
        .scroll((0, app.scroll.current))
        .block(groups_block);

    f.render_widget(group_paragraphs, chunks[0]);

    app.scroll.max = groups_y_max;

    // tasks
    // Make tasks texts
    let (task_texts, tasks_y_max): (Vec<Spans>, i16) = make_task_texts(&app, chunks[1]);

    // Draw tasks ui
    let tasks_block = Block::default()
        .title_alignment(Alignment::Center)
        .title(Spans::from(Span::styled(
            "Tasks",
            if app.window == Window::Tasks {
                Style::default().fg(Color::Black).bg(Color::White)
            } else {
                Style::default().fg(Color::White).bg(Color::Reset)
            },
        )))
        .borders(Borders::ALL);

    let task_paragraphs = Paragraph::new(task_texts)
        .scroll((0, app.task_groups[app.index].scroll.current))
        .block(tasks_block);

    f.render_widget(task_paragraphs, chunks[1]);

    app.task_groups[app.index].scroll.max = tasks_y_max;

    // If app.InputMode == Insert, then draw input ui
    let mut make_input_border = |s: String| {
        let input_border = Block::default()
            .title_alignment(Alignment::Center)
            .title(s)
            .borders(Borders::ALL);
        let area = centered_rect(40, 12, size);
        textarea.set_block(input_border);
        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(textarea.widget(), area);
    };
    match &app.input_mode {
        InputMode::Normal => {}
        InputMode::Insert(position) => match position {
            InsertPosistion::Current => make_input_border(format!(
                "Rename {}",
                match app.window {
                    Window::Tasks => "task",
                    Window::Groups => "group",
                }
            )),
            _ => make_input_border(format!(
                "Add new {}",
                match app.window {
                    Window::Tasks => "task",
                    Window::Groups => "group",
                }
            )),
        },
    }
}

fn get_showing_range(len: usize, window_height: u16, index: usize) -> (usize, usize, usize) {
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
        } else {
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
