use rusqlite::{params, Connection};

use crate::models::{State, Task};

pub fn get_tasks(
    conn: &Connection,
    group_name: &str,
) -> rusqlite::Result<Vec<Task>, rusqlite::Error> {
    let mut stmt = conn.prepare(&format!("SELECT * FROM '{}' ORDER BY id", group_name))?;
    let into_state = |f: String| match &f as &str {
        "Todo" => State::Todo,
        "Done" => State::Done,
        "Abandon" => State::Abandon,
        _ => State::Todo,
    };
    let tasks_iter = stmt
        .query_map([], |row| {
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
) -> rusqlite::Result<(), rusqlite::Error> {
    conn.execute(
        &format!(
            "
        INSERT OR IGNORE INTO {} (
            id,
            depth,
            content,
            task_state,
            group_state,
            create_time
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6);",
            group_name
        ),
        params![
            task_index,
            task.depth,
            task.content,
            format!("{:?}", task.task_state),
            format!("{:?}", task.group_state),
            task.create_time
        ],
    )?;

    // Change other tasks id
    for index in task_index + 1..tasks.len() {
        conn.execute(
            "UPDATE groups SET id = ?1 WHERE create_time = ?2",
            params![index, tasks[index].create_time],
        )
        .unwrap();
    }

    Ok(())
}

pub fn update_task(
    conn: &Connection,
    group_name: &str,
    task: &Task,
) -> rusqlite::Result<(), rusqlite::Error> {
    conn.execute(
        &format!(
            "UPDATE {} SET content = '{}' WHERE create_time = '{}';",
            group_name, task.content, task.create_time,
        ),
        [],
    )
    .unwrap();

    Ok(())
}

pub fn delete_task(
    conn: &Connection,
    tasks_len: usize,
    group_name: &str,
    task_index: usize,
    task_create_time: i64,
) -> rusqlite::Result<(), rusqlite::Error> {
    conn.execute(
        &format!("DELETE FROM {} WHERE create_time = ?1;", group_name),
        params![task_create_time],
    )?;

    // Update group_id
    for index in task_index + 1..tasks_len {
        conn.execute(
            &format!("UPDATE {} SET id = ?1 WHERE create_time = ?2;", group_name),
            params![index - 1, task_create_time,],
        )?;
    }

    Ok(())
}

pub fn change_task_state(
    conn: &Connection,
    group_name: &str,
    new_task_state: State,
    task_create_time: i64,
) -> rusqlite::Result<(), rusqlite::Error> {
    conn.execute(
        &format!(
            "UPDATE {} SET task_state = ?1 WHERE create_time = ?2",
            group_name
        ),
        params![format!("{:?}", new_task_state), task_create_time],
    )?;
    Ok(())
}
