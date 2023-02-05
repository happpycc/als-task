use rusqlite::{Connection, params};

use crate::models::{Task, State};

pub fn get_tasks(conn: &Connection, group_name: &str)
    -> rusqlite::Result<Vec<Task>, rusqlite::Error>
{
    let mut stmt = conn.prepare(
        &format!(
            "SELECT * FROM '{}' ORDER BY id",
            group_name
        )
    )?;
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

pub fn insert_task(
    conn: &Connection,
    group_name: &str,
    task: &Task,
    tasks: &Vec<Task>,
    task_index: usize,
)
    // Add new task 
    -> rusqlite::Result<(), rusqlite::Error> 
{
    conn.execute(&format!("
        INSERT OR IGNORE INTO {} (
            id,
            depth,
            content,
            task_state,
            group_state,
            create_time
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6);", group_name),
    params![
        task_index,
        task.depth,
        task.content,
        format!("{:?}", task.task_state),
        format!("{:?}", task.group_state),
        task.create_time
    ])?;

    // Change other tasks id 
    for index in task_index + 1..tasks.len() {
        conn.execute(
            "UPDATE groups SET id = ?1 WHERE create_time = ?2",
            params![index, tasks[index].create_time])
        .unwrap();
    }

    Ok(())
}

pub fn update_task() {

}

pub fn delete_task() {
    
}
