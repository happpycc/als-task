use tui::layout::{Rect, Layout, Direction, Constraint};

pub fn make_layout(size: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(75),
            ]
                .as_ref(),
        )
        .split(size)
}
