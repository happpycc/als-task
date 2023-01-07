use std::io;

use crossterm::event::{Event, self, KeyCode};
use tui::{backend::Backend, Terminal};

use crate::models::{App, InputMode};
use crate::ui::ui;


pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            ui(f, &mut app);
        })?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    _ => {}
                },
                InputMode::Insert => match key.code {
                    _ => {},
                }
            }
        }
    }
}
