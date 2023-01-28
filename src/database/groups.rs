use rusqlite::{Connection, params};
use chrono::Local;

use crate::models::TaskGroup;


pub fn init_groups(conn: &Connection)
    -> rusqlite::Result<(), rusqlite::Error> 
{
    // create a table named groups to save task_group info 
    conn.execute(
        "CREATE TABLE IF NOT EXISTS groups (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            group_id INTEGER,
            name TEXT UNIQUE,
            create_time TEXT
        );", [])
    .unwrap();

    // insert a default task_group named homeless 
    insert_group(
        conn,
        &TaskGroup { name: "homeless".to_owned(), ..Default::default() },
        &vec![],
        0,
        0,
    ).unwrap();

    Ok(())
}

pub fn insert_group(
    conn: &Connection,
    group: &TaskGroup,
    groups: &Vec<TaskGroup>,
    group_index: usize,
    groups_len: usize,
)
    -> rusqlite::Result<(), rusqlite::Error> 
{
    // Add new group info into groups(table) 
    conn.execute("
        INSERT OR IGNORE INTO groups (
            group_id,
            name,
            create_time
        ) VALUES (?1, ?2, ?3);",
    params![
        group_index,
        group.name,
        Local::now().timestamp().to_string(),
    ])?;

    // Change groups id in groups(table)
    for index in group_index + 1..groups_len {
        conn.execute(
            "UPDATE groups SET group_id = ?1 WHERE create_time = ?2",
            params![index, groups[index].create_time])
        .unwrap();
    }

    // Create new group into database 
    conn.execute(&format!(
        "CREATE TABLE IF NOT EXISTS '{}' (
            id INTEGER PRIMARY KEY,
            depth INTEGER,
            content TEXT,
            task_state TEXT,
            group_state TEXT,
            create_time TEXT
        );", group.name), [])?;

    Ok(())
}

pub fn update_group(
    conn: &Connection,
    group: &TaskGroup,
    old_name: &str
)
    -> rusqlite::Result<(), rusqlite::Error> 
{
    conn.execute(&format!(
        "UPDATE groups SET name = '{}' WHERE create_time = '{}';",
        group.name,
        group.create_time,
    ), [])
    .unwrap();

    conn.execute(&format!(
        "ALTER TABLE '{}' RENAME TO '{}'",
        old_name,
        group.name
    ),
    []).unwrap();

    Ok(())
}

pub fn delete_group(
    conn: &Connection,
    group: &TaskGroup,
    groups: &Vec<TaskGroup>,
    group_index: usize,
    groups_len: usize,
)
    -> rusqlite::Result<(), rusqlite::Error> 
{
    // Update group_id 
    for index in group_index + 1..groups_len {
        conn.execute(
            "UPDATE groups SET group_id = ?1 WHERE create_time = ?2;",
            params![
                index - 1,
                groups[index].create_time
            ])?;
    }

    // Delete current text in table 
    conn.execute(
        "DELETE FROM groups WHERE create_time = ?1;",
        params![group.create_time])?;

    // Delete current table in database 
    conn.execute(&format!(
        "DROP TABLE '{}';",
        group.name),
        [])?;

    Ok(())
}
