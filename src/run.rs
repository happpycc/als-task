use std::io;

use crossterm::event::{self, Event, KeyCode};
use tui::{backend::Backend, Terminal};
use tui_textarea::{CursorMove, TextArea};

use crate::models::{App, InputMode, InsertPosistion, Window};
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
                Window::Groups => match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('h') => app.scroll_left(),
                        KeyCode::Char('j') => app.index_next(),
                        KeyCode::Char('k') => app.index_prev(),
                        KeyCode::Char('l') => app.scroll_right(),
                        KeyCode::Char('H') => app.window_change(),
                        KeyCode::Char('L') => app.window_change(),
                        KeyCode::Char('o') => {
                            app.add_brother_next();
                            app.input_mode = InputMode::Insert(InsertPosistion::Next);
                        }
                        KeyCode::Char('O') => {
                            app.add_brother_prev();
                            app.input_mode = InputMode::Insert(InsertPosistion::Previous);
                        }
                        KeyCode::Char('i') => {
                            textarea.insert_str(&app.task_groups[app.index].name);
                            textarea.move_cursor(CursorMove::Head);
                            app.input_mode = InputMode::Insert(InsertPosistion::Current);
                        }
                        KeyCode::Char('a') => {
                            textarea.insert_str(&app.task_groups[app.index].name);
                            textarea.move_cursor(CursorMove::End);
                            app.input_mode = InputMode::Insert(InsertPosistion::Current);
                        }
                        KeyCode::Char('s') => {
                            textarea.delete_line_by_head();
                            app.input_mode = InputMode::Insert(InsertPosistion::Current);
                        }
                        KeyCode::Char('d') => app.delete_current(),
                        KeyCode::Enter => {
                            app.add_brother_next();
                            app.input_mode = InputMode::Insert(InsertPosistion::Next);
                        }
                        KeyCode::Char('q') => return Ok(()),
                        _ => {}
                    },
                    InputMode::Insert(_) => match key.code {
                        KeyCode::Enter => {
                            app.add_finished(&textarea.lines());
                            textarea.move_cursor(CursorMove::End);
                            textarea.delete_line_by_head();
                        }
                        KeyCode::Esc => {
                            app.add_abandoned();
                            textarea.move_cursor(CursorMove::End);
                            textarea.delete_line_by_head();
                        }
                        _ => {
                            textarea.input(key);
                        }
                    },
                },
                Window::Tasks => {
                    let task_group = &mut app.task_groups[app.index];
                    match app.input_mode {
                        InputMode::Normal => match key.code {
                        KeyCode::Char('h') => task_group.scroll_left(),
                        KeyCode::Char('j') => task_group.index_next(),
                        KeyCode::Char('k') => task_group.index_prev(),
                        KeyCode::Char('l') => task_group.scroll_right(),
                            KeyCode::Char('H') => {
                                app.window_change();
                            }
                            KeyCode::Char('L') => {
                                app.window_change();
                            }
                            KeyCode::Char('o') => {
                                task_group.add_brother_next(if task_group.tasks.len() == 0 {
                                    0
                                } else {
                                    task_group.tasks[task_group.index].depth
                                });
                                app.input_mode = InputMode::Insert(InsertPosistion::Next);
                            }
                            KeyCode::Char('O') => {
                                task_group.add_brother_prev();
                                app.input_mode = InputMode::Insert(InsertPosistion::Previous);
                            }
                            KeyCode::Char('i') => {
                                textarea.insert_str(&task_group.tasks[task_group.index].content);
                                textarea.move_cursor(CursorMove::Head);
                                app.input_mode = InputMode::Insert(InsertPosistion::Current);
                            }
                            KeyCode::Char('a') => {
                                textarea.insert_str(&task_group.tasks[task_group.index].content);
                                textarea.move_cursor(CursorMove::End);
                                app.input_mode = InputMode::Insert(InsertPosistion::Current);
                            }
                            KeyCode::Char('s') => {
                                textarea.delete_line_by_head();
                                app.input_mode = InputMode::Insert(InsertPosistion::Current);
                            }
                            KeyCode::Char('d') => task_group.delete_current(&app.conn),
                            KeyCode::Char(' ') => {}
                            KeyCode::Enter => {
                                task_group.add_brother_next(if task_group.tasks.len() == 0 {
                                    0
                                } else {
                                    task_group.tasks[task_group.index].depth
                                });
                                app.input_mode = InputMode::Insert(InsertPosistion::Next);
                            }
                            KeyCode::Tab => {}
                            KeyCode::Char('q') => return Ok(()),
                            _ => {}
                        },
                        InputMode::Insert(_) => match key.code {
                            KeyCode::Enter => {
                                task_group.add_finished(
                                    &app.conn,
                                    &app.input_mode,
                                    &textarea.lines(),
                                );
                                textarea.move_cursor(CursorMove::End);
                                textarea.delete_line_by_head();
                                app.input_mode = InputMode::Normal;
                            }
                            KeyCode::Esc => {
                                task_group.add_abandoned();
                                textarea.move_cursor(CursorMove::End);
                                textarea.delete_line_by_head();
                            }
                            _ => {
                                textarea.input(key);
                            }
                        },
                    }
                }
            }
        }
    }
}
