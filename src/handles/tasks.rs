use chrono::Utc;

use crate::models::{Task, State};

impl Task {
    pub fn new() -> Self {
        let local_time = Utc::now().timestamp();
        Self {
            depth: 0,
            content: "".to_string(),
            task_state: State::Todo,
            group_state: State::Todo,
            create_time: local_time,
        }
    }
}

impl Default for Task {
    fn default() -> Self {
        Self::new()
    }
}
