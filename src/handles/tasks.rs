use chrono::Local;

use crate::models::{Task, TaskState};

impl Task {
    pub fn new() -> Self {
        let local_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        Self {
            depth: 0,
            content: "".to_string(),
            state: TaskState::Todo,
            create_time: local_time,
        }
    }
}

impl Default for Task {
    fn default() -> Self {
        Self::new()
    }
}
