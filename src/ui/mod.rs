
use tui::{
    backend::Backend,
    Frame, layout::{Layout, Direction, Constraint}, widgets::{Borders, Block},
};

use crate::models::App;


pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Make Layout
    let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(
        [
            Constraint::Percentage(25),
            Constraint::Percentage(75),
        ]
        .as_ref(),
    )
    .split(f.size());

    // Draw task_groups ui
    let block = Block::default().title("Block-1").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);

    // Draw tasks ui
    let block = Block::default().title("Block 2").borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
}
