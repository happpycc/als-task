use rusqlite::{Connection, params};
use chrono::Local;

use crate::models::App;


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
    app.conn.execute("
        INSERT OR IGNORE INTO groups (
            group_id,
            name,
            create_time
        ) VALUES (?1, ?2, ?3);",
    params![
        app.index,
        task_group.name,
        task_group.create_time,
    ]).unwrap();

    // Create table => task_groups
    create_group_table(&app.conn, &task_group.name).unwrap();
    Ok(())
}

pub fn init_table_groups(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS groups (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            group_id INTEGER,
            name TEXT UNIQUE,
            create_time TEXT
        );", [])
    .unwrap();

    create_group_table(conn, "homeless").unwrap();

    conn.execute("
        INSERT OR IGNORE INTO groups (
            group_id,
            name,
            create_time
        ) VALUES (?1, ?2, ?3);",
    params![
        0,
        "homeless",
        Local::now().to_string(),
    ]).unwrap();
}

pub fn create_group_table(conn: &Connection, group_name: &str)
-> rusqlite::Result<(), rusqlite::Error> 
{
    conn.execute(&format!(
        "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY,
            depth INTEGER,
            content TEXT,
            task_state TEXT,
            group_state TEXT,
            create_time TEXT
        );", group_name), [])?;
    Ok(())
}