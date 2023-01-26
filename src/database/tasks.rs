use rusqlite::{Connection, params};

use crate::models::{Task, State};

pub fn get_tasks(conn: &Connection, group_name: &str)
    -> rusqlite::Result<Vec<Task>, rusqlite::Error>
{
    let mut stmt = conn.prepare(&format!("SELECT * FROM {} ORDER BY id", group_name))?;
    let into_state = |f: String| {
        match &f as &str {
            "Todo" => State::Todo,
            "Done" => State::Done,
            "Abandon" => State::Abandon,
            _ => State::Todo,
        }
    };
    let tasks_iter = stmt.query_map([], |row| {
        Ok(Task {
            depth: row.get(1)?,
            content: row.get(2)?,
            task_state: into_state(row.get(3)?),
            group_state: into_state(row.get(4)?),
            create_time: row.get(5)?,
        })
    })?
    .into_iter();
    let mut tasks = Vec::new();
    for task in tasks_iter {
        tasks.push(task?);
    }
    Ok(tasks)
}

pub fn insert_task(conn: &Connection, group_name: &str, task: &Task, index: usize)
    -> rusqlite::Result<(), rusqlite::Error> 
{
    conn.execute(&format!("
        INSERT OR IGNORE INTO {} (
            id,
            depth,
            content,
            state,
            create_time
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6);", group_name),
    params![
        index,
        task.depth,
        task.content,
        format!("{:?}", task.task_state),
        format!("{:?}", task.group_state),
        task.create_time
    ])?;
    Ok(())
}