use tui::{text::{Spans, Span}, layout::Rect, style::{Color, Style}};
use unicode_width::UnicodeWidthStr;

use crate::models::App;

use super::get_showing_range;

pub fn make_group_texts(app: &App, size: Rect) -> (Vec<Spans>, usize) {
    let groups = &app.task_groups;

    let (begin, end, highlight_index) = get_showing_range(
        groups.len(),
        size.height,
        app.index,
    );

    let mut max_groups_len = 0;

    let texts: Vec<Spans> = groups[begin..end]
        .iter()
        .enumerate()
        .map(|(index, group)| {
            if group.name.width() > max_groups_len {max_groups_len = group.name.width()}

            let mut text_style = Style::default().fg(Color::White).bg(Color::Reset);
            if highlight_index == index {
                text_style = Style::default().fg(Color::Black).bg(Color::White);
            }
            Spans::from(Span::styled(group.name.as_str(), text_style))
        })
        .collect();

    (texts, max_groups_len)
}
