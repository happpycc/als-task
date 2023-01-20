use std::io;

use crossterm::event::{Event, self, KeyCode};
use tui::{backend::Backend, Terminal};
use tui_textarea::{TextArea, CursorMove};

use crate::models::{App, InputMode, Window, InsertPosistion};
use crate::ui::ui;


pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App
) -> io::Result<()> {
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
                            KeyCode::Char('o') => { app.add_brother_next() },
                            KeyCode::Char('O') => { app.add_brother_prev() },
                            KeyCode::Char('i') => {
                                textarea.move_cursor(CursorMove::Head);
                                app.input_mode = InputMode::Insert(InsertPosistion::Current);
                            },
                            KeyCode::Char('a') => {
                                textarea.move_cursor(CursorMove::End);
                                app.input_mode = InputMode::Insert(InsertPosistion::Current);
                            },
                            KeyCode::Char('s') => { 
                                textarea.delete_line_by_head();
                                app.input_mode = InputMode::Insert(InsertPosistion::Current);
                            },
                            KeyCode::Char('d') => { app.delete_current() },
                            KeyCode::Enter => { app.add_brother_next() },
                            KeyCode::Char('q') => return Ok(()),
                            _ => {}
                        },
                        InputMode::Insert(_) => match key.code {
                            KeyCode::Enter => {
                                app.add_finished(&textarea.lines());
                                textarea.delete_line_by_head();
                            },
                            KeyCode::Esc => {
                                app.add_abandoned();
                                textarea.delete_line_by_head();
                            },
                            _ => { textarea.input(key); },
                        }
                    }
                },
                Window::Tasks => {
                    match app.input_mode {
                        InputMode::Normal => match key.code {
                            KeyCode::Char('j') => { app.index_next()},
                            KeyCode::Char('k') => { app.index_prev()},
                            KeyCode::Char('H') => { app.window_change(); },
                            KeyCode::Char('L') => { app.window_change(); },
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
                        InputMode::Insert(_) => match key.code {
                            _ => {},
                        }
                    }
                },
            }
        }
    }
}
