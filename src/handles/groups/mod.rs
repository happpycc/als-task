use chrono::Local;
use crate::models::TaskGroup;

pub mod modify;
pub mod movement;


impl TaskGroup {
    pub fn new(name: String) -> Self {
        let local_time = Local::now().to_string();
        Self {
            name,
            tasks: vec![],
            index: 0,
            create_time: local_time,
        }
    }
}

impl Default for TaskGroup {
    fn default() -> Self {
        TaskGroup::new("".to_owned())
    }
}
