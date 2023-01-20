use rusqlite::{Connection, params};

use crate::models::{TaskGroup, Task, State, App};

struct Groups((String, String));

pub fn init_database()
-> rusqlite::Result<Connection, rusqlite::Error> 
{
    let conn = Connection::open("tasks.db").unwrap();
    init_table_groups(&conn);
    create_group_table(&conn, "homeless").unwrap();

    Ok(conn)
}


pub fn init_table_groups(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS groups (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            group_id INTEGER,
            name TEXT,
            create_time TEXT
        );", [])
    .unwrap();
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

pub fn get_all_data(conn: &Connection)
    -> rusqlite::Result<Vec<TaskGroup>, rusqlite::Error>
{
    let mut task_groups = vec![];
    for group in conn.prepare("SELECT * FROM groups ORDER BY group_id;")?
        .query_map([], |row| {
            Ok(Groups((row.get(2)?, row.get(3)?)))
        })?
    .into_iter() {
        let (name, create_time) = group?.0;
        task_groups.push(TaskGroup {
            tasks: get_tasks(&conn, &name.as_str()).unwrap(),
            name,
            index: 0,
            create_time,
        });
    }

    Ok(task_groups)
}

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
        INSERT INTO {} (
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
        INSERT INTO groups (
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
