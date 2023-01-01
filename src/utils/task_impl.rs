use chrono::Local;

use crate::models::{Task, TaskState};

impl Task {
    pub fn new() -> Self {
        let local_time = Local::now();
        Self {
            depth: 0,
            task_id: 0,
            content: "".to_string(),
            state: TaskState::default(),
            comments: None,
            create_time: local_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            update_time: local_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            dead_time: None,
        }
    }
}

impl Default for Task {
    fn default() -> Self {
        Task::new()
    }
}