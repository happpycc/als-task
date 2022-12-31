use rusqlite::Connection;
use std::io;

use crossterm::event::{self, Event, KeyCode};
use tui::{backend::Backend, Terminal};
use tui_textarea::TextArea;

use crate::model::{App, Task, TaskState};
use crate::ui::ui;
use chrono::Local;
use rusqlite::params;
use tui::layout::Rect;
use unicode_width::UnicodeWidthStr;

use crate::model::InputMode;

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
        } else {
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

pub fn load_database() -> rusqlite::Result<Connection> {
    let conn = Connection::open("tasks.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        task_id INTEGER,
        depth INTEGER,
        content TEXT,
        state TEXT,
        comments TEXT,
        create_time TEXT,
        update_time TEXT,
        dead_time TEXT
    )",
        [],
    )?;
    Ok(conn)
}

pub fn load_tasks(conn: &Connection) -> rusqlite::Result<Vec<Task>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT * FROM tasks ORDER BY task_id")?;
    let into_task_state = |f: String| match &f as &str {
        "Todo" => TaskState::Todo,
        "Done" => TaskState::Done,
        "Abandon" => TaskState::Abandon,
        _ => TaskState::Todo,
    };
    let tasks_iter = stmt
        .query_map([], |row| {
            Ok(Task {
                task_id: row.get(1)?,
                depth: row.get(2)?,
                content: row.get(3)?,
                state: into_task_state(row.get(4)?),
                comments: row.get(5)?,
                create_time: row.get(6)?,
                update_time: row.get(7)?,
                dead_time: row.get(8)?,
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
        if content == ""
            || format!("{:?} ", self.tasks[self.index].state).len() as u16
                + content.width() as u16
                + (self.tasks[self.index].depth * 4) as u16
                > self.window_rect.width
        {
            self.edit_abandon();
        } else {
            self.tasks[self.index].content = content.to_string();
            for index in (self.index + 1)..self.tasks.len() {
                self.tasks[index].task_id = index;
                self.conn
                    .execute(
                        "UPDATE tasks SET task_id = ?1 WHERE create_time = ?2",
                        params![index, self.tasks[index].create_time],
                    )
                    .unwrap();
            }
            self.tasks[self.index].task_id = self.index;
            let task = &self.tasks[self.index];
            self.conn
                .execute(
                    "
                INSERT INTO tasks (
                    task_id,
                    depth,
                    content,
                    state,
                    comments,
                    create_time,
                    update_time,
                    dead_time
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8); 
            ",
                    params![
                        task.task_id,
                        task.depth,
                        task.content,
                        format!("{:?}", task.state),
                        task.comments,
                        task.create_time,
                        task.update_time,
                        task.dead_time
                    ],
                )
                .unwrap();
        }
    }

    pub fn edit_abandon(&mut self) {
        self.tasks.remove(self.index);
        self.index -= if self.index == 0 { 0 } else { 1 };
        self.input_mode = InputMode::Normal;
    }

    fn get_next_brother_task(&self) -> usize {
        if self.tasks.len() == 0 {
            return 0;
        } else if self.index + 1 == self.tasks.len() {
            return self.tasks.len();
        } else if self.tasks[self.index + 1].depth == self.tasks[self.index].depth {
            return self.index + 1;
        }
        for index in self.index + 1..self.tasks.len() {
            if self.tasks[self.index].depth == self.tasks[index].depth {
                return index;
            } else if index == self.tasks.len() - 1 {
                return index + 1;
            } else if self.tasks[index].depth < self.tasks[self.index].depth {
                return index;
            }
        }
        0
    }

    pub fn add_brother_task(&mut self) {
        self.input_mode = InputMode::Editing;
        let old_index = self.index;
        self.index = self.get_next_brother_task();
        self.tasks.insert(
            self.index,
            Task {
                depth: if self.tasks.len() == 0 {
                    0
                } else {
                    self.tasks[old_index].depth
                },
                ..Default::default()
            },
        );
    }

    pub fn add_child_task(&mut self) {
        self.input_mode = InputMode::Editing;
        self.tasks.insert(
            if self.tasks.len() == 0 {
                0
            } else {
                self.index + 1
            },
            Task {
                depth: if self.tasks.len() == 0 {
                    0
                } else {
                    self.tasks[self.index].depth + 1
                },
                ..Default::default()
            },
        );
        self.index += if self.tasks.len() - 1 == 0 { 0 } else { 1 };
    }

    pub fn delete_task(&mut self) {
        if self.tasks.len() == 0 {
            return;
        } else if self.index == self.tasks.len() - 1 {
            self.conn
                .execute(
                    "DELETE FROM tasks WHERE task_id=?1",
                    params![self.tasks[self.index].task_id],
                )
                .unwrap();
            self.tasks.remove(self.index);
            self.index -= if self.index == 0 { 0 } else { 1 };
            return;
        } else {
            let mut delete_num = 0;
            let first_index = self.tasks[self.index].task_id - if self.index == 0 { 0 } else { 1 };
            let first_depth = self.tasks[self.index].depth;
            let mut is_delete = true;
            for index in self.index..self.tasks.len() {
                if index != self.index && self.tasks[self.index].depth <= first_depth {
                    is_delete = false;
                }
                if is_delete {
                    delete_num += 1;
                    self.conn
                        .execute(
                            "DELETE FROM tasks WHERE create_time = ?1",
                            params![self.tasks[self.index].create_time],
                        )
                        .unwrap();
                    self.tasks.remove(self.index);
                } else {
                    self.tasks[self.index].task_id -= delete_num;
                    self.conn
                        .execute(
                            "UPDATE tasks SET task_id = ?1 WHERE create_time = ?2",
                            params![
                                self.tasks[self.index].task_id,
                                self.tasks[self.index].create_time
                            ],
                        )
                        .unwrap();
                    self.index += 1;
                }
            }
            self.index = first_index;
            // for index in self.index + 1..self.tasks.len() {
            // }
        }
    }

    pub fn change_state(&mut self, state: TaskState) {
        if self.tasks.len() == 0 {
            return;
        } else if self.index + 1 == self.tasks.len() {
            return self.tasks[self.index].state = state;
        } else if self.tasks[self.index + 1].depth == self.tasks[self.index].depth {
            return self.tasks[self.index].state = state;
        }
        for index in self.index..self.tasks.len() {
            self.tasks[index].state = state;
            if self.tasks[self.index].depth == self.tasks[index + 1].depth {
                return;
            } else if self.tasks[index + 1].depth < self.tasks[self.index].depth {
                return;
            }
        }
    }

    pub fn next(&mut self) {
        if self.index
            == if self.tasks.len() != 0 {
                self.tasks.len() - 1
            } else {
                0
            }
        {
            self.index = 0;
            return;
        }
        self.index += 1;
    }

    pub fn previous(&mut self) {
        if self.index == 0 {
            self.index = if self.tasks.len() != 0 {
                self.tasks.len() - 1
            } else {
                0
            };
            return;
        }
        self.index -= 1;
    }

    // pub fn scroll_left(&mut self) {
    //     self.scroll -= 1;
    // }

    // pub fn scroll_right(&mut self) {
    //     self.scroll += 1;
    // }
}

impl Task {
    pub fn new() -> Self {
        let local_time = Local::now();
        Self {
            depth: 0,
            task_id: 0,
            content: "".to_string(),
            state: TaskState::default(),
            comments: None,
            create_time: local_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            update_time: local_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            dead_time: None,
        }
    }
}

impl Default for Task {
    fn default() -> Self {
        Task::new()
    }
}

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
                    KeyCode::Char(' ') => match app.tasks[app.index].state {
                        TaskState::Todo => app.change_state(TaskState::Done),
                        TaskState::Done => app.change_state(TaskState::Todo),
                        TaskState::Abandon => app.change_state(TaskState::Todo),
                    },
                    KeyCode::Char('x') => app.change_state(TaskState::Abandon),
                    KeyCode::Char('d') => app.delete_task(),
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Enter => {
                        app.add_brother_task();
                        textarea.delete_line_by_head();
                    }
                    KeyCode::Tab => {
                        app.add_child_task();
                        textarea.delete_line_by_head();
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => app.edit_finished(textarea.lines()),
                    KeyCode::Esc => app.edit_abandon(),
                    _ => {
                        textarea.input(key);
                    }
                },
            }
        }
    }
}