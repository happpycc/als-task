use rusqlite::{Connection, params};

use crate::models::{TaskGroup, Task, TaskState};

struct TableName(String);

pub fn init_database()
-> rusqlite::Result<Connection, rusqlite::Error> 
{
    let conn = Connection::open("tasks.db").unwrap();
    create_group(&conn, "homeless").unwrap();

    Ok(conn)
}

pub fn create_group(conn: &Connection, group_name: &str)
-> rusqlite::Result<(), rusqlite::Error> 
{
    conn.execute(&format!("CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY,
            depth INTEGER,
            content TEXT,
            state TEXT,
            create_time TEXT
        );", group_name), [])?;
    Ok(())
}

pub fn get_all_data(conn: &Connection)
    -> rusqlite::Result<Vec<TaskGroup>, rusqlite::Error>
{
    let mut task_groups = vec![];
    for table_name in conn.prepare("SELECT * FROM sqlite_master WHERE type='table';")?
        .query_map([], |row| {
            Ok(TableName(row.get(1)?))
        })?
    .into_iter() {
        let table_name = table_name?.0;
        task_groups.push(TaskGroup {
            tasks: get_tasks(&conn, &table_name.as_str()).unwrap(),
            name: table_name,
            index: 0
        });
    }

    Ok(task_groups)
}

pub fn get_tasks(conn: &Connection, group_name: &str)
    -> rusqlite::Result<Vec<Task>, rusqlite::Error>
{
    let mut stmt = conn.prepare(&format!("SELECT * FROM {} ORDER BY id", group_name))?;
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
            depth: row.get(2)?,
            content: row.get(3)?,
            state: into_task_state(row.get(4)?),
            create_time: row.get(6)?,
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
            create_time,
        ) VALUES (?1, ?2, ?3, ?4, ?5);", group_name),
    params![
        index,
        task.depth,
        task.content,
        format!("{:?}", task.state),
        task.create_time
    ])?;
    Ok(())
}
