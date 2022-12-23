use std::io;

use crossterm::event::{KeyCode, self, Event};
use tui::{backend::Backend, Terminal};
use tui_textarea::TextArea;

use crate::model::{App, InputMode};
use crate::ui::ui;

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let mut textarea = TextArea::default();

    loop {
        terminal.draw(|f| {
            ui(f, &app, &mut textarea);
            app.window_rect = f.size();
        })?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('h') | KeyCode::Char('k') | KeyCode::Up => { app.previous(); },
                    KeyCode::Char('l') | KeyCode::Char('j') | KeyCode::Down => { app.next(); },
                    KeyCode::Char(' ') => app.tasks[app.index].todo_or_done(),
                    KeyCode::Char('x') => app.tasks[app.index].abandon(),
                    KeyCode::Char('d') => app.delete_task(),
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Enter => { app.add_brother_task(); textarea.delete_line_by_head(); },
                    KeyCode::Tab => { app.add_child_task(); textarea.delete_line_by_head(); },
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => app.edit_finished(textarea.lines()),
                    KeyCode::Esc => app.edit_abandon(),
                    _ => {textarea.input(key);},
                }
            }
        }
    }
}