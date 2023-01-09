use crate::models::{App, InputMode, ScrollY};
use crate::database::{init_database, get_all_data};


impl App {
    pub fn new() -> App {
        let conn = init_database().unwrap();
        let task_groups = get_all_data(&conn).unwrap();
        App {
            conn,
            task_groups,
            input_mode: InputMode::Normal,
            index: 0,
            scroll_task: ScrollY { current: 0, max: 0 },
            scroll_group: ScrollY { current: 0, max: 0 },
        }
    }
}
