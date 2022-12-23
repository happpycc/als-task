use std::io;

use crossterm::event::{KeyCode, self, Event};
use tui::{backend::Backend, Terminal};
use tui::layout::Rect;
use tui_textarea::TextArea;
use snowflake::SnowflakeIdBucket;
use chrono::prelude::Local;
use unicode_width::UnicodeWidthStr;

use crate::model::{App, InputMode, Task, TaskState};
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

impl App {
    pub fn new() -> App {
        App {
            tasks: vec![],
            index: 0,
            input_mode: InputMode::Normal,
            window_rect: Rect::default(),
        }
    }

    pub fn edit_finished(&mut self, content: &[String]) {
        let content = &content[0];
        self.input_mode = InputMode::Normal;
        if content == "" || format!("{:?} ", self.tasks[self.index].state).len() as u16 + content.width() as u16 + (self.tasks[self.index].depth * 4) as u16 > self.window_rect.width {
            self.edit_abandon();
        } else {
            self.tasks[self.index].content = content.to_string();
        }
    }

    pub fn edit_abandon(&mut self) {
        self.tasks.remove(self.index);
        self.index -= if self.index == 0 {0} else {1};
        self.input_mode = InputMode::Normal;
    }

    pub fn add_brother_task(&mut self) {
        self.input_mode = InputMode::Editing;
        self.tasks.insert(if self.tasks.len() == 0 {0} else {self.index + 1}, Task { depth: if self.tasks.len() == 0 {0} else {self.tasks[self.index].depth}, ..Default::default() });
        self.index += if self.tasks.len() - 1 == 0 {0} else {1};
    }

    pub fn add_child_task(&mut self) {
        self.input_mode = InputMode::Editing;
        self.tasks.insert(if self.tasks.len() == 0 {0} else {self.index + 1}, Task { depth: if self.tasks.len() == 0 {0} else {self.tasks[self.index].depth + 1}, ..Default::default() });
        self.index += if self.tasks.len() - 1 == 0 {0} else {1};
    }

    pub fn delete_task(&mut self) {
        if self.tasks.len() != 0 {
            self.tasks.remove(self.index);
            self.index -= if self.index == 0 {0} else {1};
        }
    }

    pub fn next(&mut self) {
        if self.index == self.tasks.len() - 1 {
            self.index = 0;
            return;
        }
        self.index += 1;
    }

    pub fn previous(&mut self) {
        if self.index == 0 {
            self.index = self.tasks.len() - 1;
            return;
        }
        self.index -= 1;
    }
}

impl Task {
    pub fn new() -> Self {
        let mut id_generator_bucket = SnowflakeIdBucket::new(1, 1);
        let local_time = Local::now();
        Self {
            depth: 0,
            task_id: id_generator_bucket.get_id(),
            content: "".to_string(),
            state: TaskState::default(),
            comments: None,
            create_time: local_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            update_time: local_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            dead_time: None,
            prev_task: None,
            next_task: None
        }
    }
    pub fn todo_or_done(&mut self) {
        match &self.state {
            TaskState::Todo => {self.state = TaskState::Done},
            TaskState::Done => {self.state = TaskState::Todo},
            TaskState::Abandon => {self.state = TaskState::Todo},
        }
        
    }
    pub fn abandon(&mut self) {
        self.state = TaskState::Abandon;
    }

}

impl Default for Task {
    fn default() -> Self {
        Task::new()
    }
}