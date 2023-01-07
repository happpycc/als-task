use rusqlite::{Connection, params};

use crate::models::TaskGroup;

pub fn init_database() -> rusqlite::Result<Connection, rusqlite::Error> {
    let conn = Connection::open("tasks.db").unwrap();
    create_group(&conn, "homeless").unwrap();

    Ok(conn)
}

pub fn create_group(conn: &Connection, group_name: &str)
    -> rusqlite::Result<(), rusqlite::Error> 
{
    conn.execute("CREATE TABLE IF NOT EXISTS tasks (
        id INTEGER PRIMARY KEY,
        depth INTEGER,
        content TEXT,
        state TEXT,
        create_time TEXT
    );", params![group_name])?;
    Ok(())
}

pub fn get_all_data(conn: &Connection) -> Vec<TaskGroup> {
    vec![]
}
