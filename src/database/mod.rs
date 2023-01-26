use rusqlite::Connection;

use crate::models::TaskGroup;

use self::{groups::init_table_groups, tasks::get_tasks};

pub mod groups;
pub mod tasks;

struct Groups((String, String));

pub fn init_database()
-> rusqlite::Result<Connection, rusqlite::Error> 
{
    let conn = Connection::open("tasks.db").unwrap();
    init_table_groups(&conn);

    Ok(conn)
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