use std::io;

use crossterm::event::{KeyCode, self, Event};
use rusqlite::{Connection, params};
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
            app.window_rect = f.size();
            ui(f, &app, &mut textarea);
        })?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('k') | KeyCode::Up => app.previous(),
                    KeyCode::Char('j') | KeyCode::Down => app.next(),
                    // KeyCode::Char('h') | KeyCode::Left => app.scroll_left(),
                    // KeyCode::Char('l') | KeyCode::Right => app.scroll_right(),
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

pub fn get_showing_tasks(app: &App) -> (usize, usize, usize) {
    let begin: usize;
    let end: usize;
    let highlight_index: usize;
    let can_showed_num = app.window_rect.height as usize - 2;
    let tasks_len = app.tasks.len();
    // within window
    if can_showed_num >= tasks_len {
        begin = 0;
        end = tasks_len;
        highlight_index = app.index;
    } else {
    // out of window
        if app.index as isize <= can_showed_num as isize / 2 - 1 {
            begin = 0;
            end = can_showed_num;
            highlight_index = app.index;
        } else if tasks_len - can_showed_num + can_showed_num / 2 <= app.index {
            begin = tasks_len - can_showed_num;
            end = tasks_len;
            highlight_index = app.index - (tasks_len - can_showed_num);
        }
        else {
            if can_showed_num > 1 {
                if can_showed_num % 2 == 0 {
                    begin = app.index - (can_showed_num / 2 - 1);
                    end = app.index + (can_showed_num / 2) + 1;
                    highlight_index = can_showed_num / 2 - 1;
                } else {
                    begin = app.index - (can_showed_num - 1) / 2;
                    end = app.index + (can_showed_num - 1) / 2 + 1;
                    highlight_index = (can_showed_num + 1) / 2 - 1;
                }
            } else {
                begin = app.index;
                end = app.index + 1;
                highlight_index = 0;
            }
        }
    }
    (begin, end, highlight_index)
}

fn load_database() -> rusqlite::Result<Connection> {
    let conn = Connection::open("tasks.db")?;
    conn.execute("CREATE TABLE IF NOT EXISTS tasks (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        task_id INTEGER,
        depth INTEGER,
        content TEXT,
        state TEXT,
        comments TEXT,
        create_time TEXT,
        update_time TEXT,
        dead_time TEXT,
        next_task TEXT
    )", [])?;
    Ok(conn)
}

fn load_tasks(conn: &Connection) -> rusqlite::Result<Vec<Task>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT * FROM tasks")?;
    let into_task_state = |f: String| {
        match &f as &str {
            "Todo" => TaskState::Todo,
            "Done" => TaskState::Done,
            "Abandon" => TaskState::Abandon,
            _ => TaskState::Todo,
        }
    };
    let tasks_iter = stmt.query_map([], |row| {
        Ok(Task {
            task_id: row.get(1)?,
            depth: row.get(2)?,
            content: row.get(3)?,
            state: into_task_state(row.get(4)?),
            comments: row.get(5)?,
            create_time: row.get(6)?,
            update_time: row.get(7)?,
            dead_time: row.get(8)?,
            next_task: row.get(9)?,
        })
    })?
    .into_iter();
    let mut tasks = Vec::new();
    for task in tasks_iter {
        tasks.push(task?);
    }
    Ok(tasks)
}

impl App {
    pub fn new() -> App {
        let conn = load_database().unwrap();
        let tasks = load_tasks(&conn).unwrap();
        App {
            tasks,
            index: 0,
            input_mode: InputMode::Normal,
            window_rect: Rect::default(),
            scroll: 0,
            conn,
        }
    }


    pub fn edit_finished(&mut self, content: &[String]) {
        let content = &content[0];
        self.input_mode = InputMode::Normal;
        if content == "" || format!("{:?} ", self.tasks[self.index].state).len() as u16 + content.width() as u16 + (self.tasks[self.index].depth * 4) as u16 > self.window_rect.width {
            self.edit_abandon();
        } else {
            self.tasks[self.index].next_task = if self.index + 1 < self.tasks.len() {Some(self.tasks[self.index + 1].task_id)} else {None};
            self.tasks[self.index].content = content.to_string();
            let task = &self.tasks[self.index];
            self.conn.execute("
                INSERT INTO tasks (
                    task_id,
                    depth,
                    content,
                    state,
                    comments,
                    create_time,
                    update_time,
                    dead_time,
                    next_task
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9); 
            ", params![task.task_id, task.depth, task.content, format!("{:?}", task.state), task.comments,
            task.create_time, task.update_time, task.dead_time, task.next_task]).unwrap();
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
            self.conn.execute("DELETE FROM tasks WHERE task_id=?1", params![self.tasks[self.index].task_id]).unwrap();
            self.tasks.remove(self.index);
            if self.index != 0 && self.index < self.tasks.len() - 1 {
                self.tasks[self.index - 1].task_id = self.tasks[self.index + 1].task_id;
            }
            self.index -= if self.index == 0 {0} else {1};
        }
    }

    pub fn next(&mut self) {
        if self.index == if self.tasks.len() != 0 {self.tasks.len() - 1} else {0} {
            self.index = 0;
            return;
        }
        self.index += 1;
    }

    pub fn previous(&mut self) {
        if self.index == 0 {
            self.index = if self.tasks.len() != 0 {self.tasks.len() - 1} else {0};
            return;
        }
        self.index -= 1;
    }


    pub fn scroll_left(&mut self) {
        self.scroll -= 1;
    }

    pub fn scroll_right(&mut self) {
        self.scroll += 1;
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