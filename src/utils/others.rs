use rusqlite::Connection;

use crate::models::{App, Task, TaskState};

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

pub fn load_database() -> rusqlite::Result<Connection> {
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
        dead_time TEXT
    )", [])?;
    Ok(conn)
}

pub fn load_tasks(conn: &Connection) -> rusqlite::Result<Vec<Task>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT * FROM tasks ORDER BY task_id")?;
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
        })
    })?
    .into_iter();
    let mut tasks = Vec::new();
    for task in tasks_iter {
        tasks.push(task?);
    }
    Ok(tasks)
}