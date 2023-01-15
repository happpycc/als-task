use tui::{text::{Spans, Span}, layout::Rect, style::{Style, Color}};
use unicode_width::UnicodeWidthStr;

use crate::models::App;
use super::get_showing_range;


pub fn make_task_texts(app: &App, size: Rect) -> (Vec<Spans>, usize) {
    if app.task_groups.len() == 0 { return (vec![], 0) }
    let tasks = &app.task_groups[app.index].tasks;

    let (begin, end, highlight_index) = get_showing_range(
        tasks.len(),
        size.height,
        app.task_groups[app.index].index
    );

    let mut max_tasks_len = 0;
    let texts: Vec<Spans> = tasks[begin..end]
        .iter()
        .enumerate()
        .map(|(index, task)| {
            if task.content.width() > max_tasks_len {max_tasks_len = task.content.width()}

            let task_depth = format!("{:1$}", "", (task.depth * 4) as usize);
            let task_state = format!("{:?} ", task.task_state);
            let mut text_style = Style::default().fg(Color::White).bg(Color::Reset);
            if highlight_index == index {
                text_style = Style::default().fg(Color::Black).bg(Color::White);
            }
            Spans::from(vec![
                Span::raw(task_depth),
                Span::raw(task_state),
                Span::styled(task.content.as_str(), text_style),
            ])
        })
        .collect();

    (texts, max_tasks_len)
}
