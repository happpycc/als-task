use std::io;

use crossterm::event::{self, Event, KeyCode};
use tui::{backend::Backend, Terminal};
use tui_textarea::TextArea;

use crate::{models::{App, InputMode, TaskState}, ui::ui};


pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let mut textarea = TextArea::default();

    loop {
        terminal.draw(|f| {
            app.window_rect = f.size();
            app.scroll_right_max = 0;
            ui(f, &mut app, &mut textarea);
        })?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('k') | KeyCode::Up => app.previous(),
                    KeyCode::Char('j') | KeyCode::Down => app.next(),
                    KeyCode::Char('h') | KeyCode::Left => app.scroll_left(),
                    KeyCode::Char('l') | KeyCode::Right => app.scroll_right(),
                    KeyCode::Char(' ') => match app.tasks[app.index].state {
                        TaskState::Todo => app.change_state(TaskState::Done),
                        TaskState::Done => app.change_state(TaskState::Todo),
                        TaskState::Abandon => app.change_state(TaskState::Todo),
                    } ,
                    KeyCode::Char('x') => app.change_state(TaskState::Abandon),
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