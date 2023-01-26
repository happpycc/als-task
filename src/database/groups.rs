use rusqlite::{Connection, params, named_params};
use chrono::Local;

use crate::models::{App, TaskGroup};


pub fn init_table_groups(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS groups (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            group_id INTEGER,
            name TEXT UNIQUE,
            create_time TEXT
        );", [])
    .unwrap();

    insert_into_groups(conn, 0, "homeless").unwrap();

    create_table_group(conn, "homeless").unwrap();
}

pub fn create_table_group(conn: &Connection, group_name: &str)
-> rusqlite::Result<(), rusqlite::Error> 
{
    conn.execute(&format!(
        "CREATE TABLE IF NOT EXISTS '{}' (
            id INTEGER PRIMARY KEY,
            depth INTEGER,
            content TEXT,
            task_state TEXT,
            group_state TEXT,
            create_time TEXT
        );", group_name), [])?;
    Ok(())
}

pub fn insert_group(app: &App)
    -> rusqlite::Result<(), rusqlite::Error> 
{
    // Change group_id
    for index in app.index + 1..app.task_groups.len() {
        app.conn.execute(
            "UPDATE groups SET group_id = ?1 WHERE create_time = ?2",
            params![index, app.task_groups[index].create_time])
        .unwrap();
    }

    // Add task_groups into groups table
    let task_group = &app.task_groups[app.index];

    insert_into_groups(&app.conn, app.index, &task_group.name).unwrap();

    // Create table => task_groups
    create_table_group(&app.conn, &task_group.name).unwrap();
    Ok(())
}

pub fn insert_into_groups(conn: &Connection, index: usize, name: &str)
    -> rusqlite::Result<(), rusqlite::Error> 
{
    conn.execute("
        INSERT OR IGNORE INTO groups (
            group_id,
            name,
            create_time
        ) VALUES (?1, ?2, ?3);",
    params![
        index,
        name,
        Local::now().to_string(),
    ])?;

    Ok(())
}

pub fn update_groups_name(conn: &Connection, group: &TaskGroup)
    -> rusqlite::Result<(), rusqlite::Error> 
{
    conn.execute(&format!(
        "UPDATE groups SET name = '{}' WHERE create_time = '{}';",
        group.name,
        group.create_time,
    ), [])
    .unwrap();

    Ok(())
}
