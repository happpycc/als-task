use tui::{text::{Spans, Span}, layout::Rect, style::{Color, Style}};
use unicode_width::UnicodeWidthStr;

use crate::models::App;

use super::get_showing_range;

pub fn make_group_texts(app: &App, size: Rect) -> (Vec<Spans>, i16) {
    let groups = &app.task_groups;

    let (begin, end, highlight_index) = get_showing_range(
        groups.len(),
        size.height,
        app.index,
    );

    let mut group_y_max = app.scroll.max;

    let texts: Vec<Spans> = groups[begin..end]
        .iter()
        .enumerate()
        .map(|(index, group)| {
            let scroll_len = {
                group.name.width() as i16
                    - size.width as i16
                    + 2
            };
            if scroll_len > group_y_max {
                group_y_max = scroll_len;
            }

            let mut text_style = Style::default().fg(Color::White).bg(Color::Reset);
            if highlight_index == index {
                text_style = Style::default().fg(Color::Black).bg(Color::White);
            }
            Spans::from(Span::styled(group.name.as_str(), text_style))
        })
        .collect();

    (texts, group_y_max)
}
