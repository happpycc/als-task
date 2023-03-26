use tui::{text::{Spans, Span}, layout::Rect, style::{Style, Color}};
use unicode_width::UnicodeWidthStr;

use crate::models::App;
use super::get_showing_range;


pub fn make_task_texts(app: &App, size: Rect) -> (Vec<Spans>, i16) {
    if app.task_groups.len() == 0 { return (vec![], 0) }
    let group = &app.task_groups[app.index];

    let (begin, end, highlight_index) = get_showing_range(
        group.tasks.len(),
        size.height,
        group.index
    );

    let mut tasks_y_max = group.scroll.max;
    let texts: Vec<Spans> = group.tasks[begin..end]
        .iter()
        .enumerate()
        .map(|(index, task)| {
            let task_depth = format!("{:1$}", "", (task.depth * 4) as usize);
            let task_state = format!("{:?} ", task.task_state);

            let scroll_len = (
                    task_depth.len()
                    + task_state.len()
                    + task.content.width()
                ) as i16
                    - size.width as i16
                    + 2;
            if scroll_len > tasks_y_max { tasks_y_max = scroll_len }

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

    (texts, tasks_y_max)
}
