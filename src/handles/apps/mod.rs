use crate::models::{App, SeletedIndex, InputMode, ScrollY};
use crate::database::{init_database, get_all_data};


impl App {
    pub fn new() -> App {
        let conn = init_database().unwrap();
        let task_groups = get_all_data(&conn);
        App {
            conn,
            task_groups,
            input_mode: InputMode::Normal,
            index: SeletedIndex { task: 0, group: 0 },
            scroll_task: ScrollY { current: 0, max: 0 },
            scroll_group: ScrollY { current: 0, max: 0 },
        }
    }
}
