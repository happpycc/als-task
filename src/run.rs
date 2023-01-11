use std::io;

use crossterm::event::{Event, self, KeyCode};
use tui::{backend::Backend, Terminal};
use tui_textarea::TextArea;

use crate::models::{App, InputMode, Window};
use crate::ui::ui;


pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    // Init textarea
    let mut textarea = TextArea::default();

    loop {
        terminal.draw(|f| {
            ui(f, &mut app, &mut textarea);
        })?;

        if let Event::Key(key) = event::read()? {
            match app.window {
                Window::Groups => {
                    match app.input_mode {
                        InputMode::Normal => match key.code {
                            KeyCode::Char('j') => { app.index_next() },
                            KeyCode::Char('k') => { app.index_prev() },
                            KeyCode::Char('H') => { app.window_change() },
                            KeyCode::Char('L') => { app.window_change() },
                            KeyCode::Char('o') => {},
                            KeyCode::Char('O') => {},
                            KeyCode::Char('i') => {},
                            KeyCode::Char('a') => {},
                            KeyCode::Char('s') => {},
                            KeyCode::Char('d') => {},
                            KeyCode::Enter => {},
                            KeyCode::Char('q') => return Ok(()),
                            _ => {}
                        },
                        InputMode::Insert => match key.code {
                            _ => {},
                        }
                    }
                },
                Window::Tasks => {
                    match app.input_mode {
                        InputMode::Normal => match key.code {
                            KeyCode::Char('j') => { app.index_next()},
                            KeyCode::Char('k') => { app.index_prev()},
                            KeyCode::Char('H') => { app.window_change() },
                            KeyCode::Char('L') => { app.window_change() },
                            KeyCode::Char('o') => {},
                            KeyCode::Char('O') => {},
                            KeyCode::Char('i') => {},
                            KeyCode::Char('a') => {},
                            KeyCode::Char('s') => {},
                            KeyCode::Char('d') => {},
                            KeyCode::Char(' ') => {},
                            KeyCode::Enter => {},
                            KeyCode::Tab => {},
                            KeyCode::Char('q') => return Ok(()),
                            _ => {}
                        },
                        InputMode::Insert => match key.code {
                            _ => {},
                        }
                    }
                },
            }
        }
    }
}
