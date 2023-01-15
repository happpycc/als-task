use chrono::Local;
use crate::models::TaskGroup;


impl TaskGroup {
    pub fn new(name: String) -> Self {
        let local_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        Self {
            name,
            tasks: vec![],
            index: 0,
            create_time: local_time,
        }
    }
}
