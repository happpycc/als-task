use chrono::prelude::Local;
use serde::{Deserialize, Serialize};
use serde_json::{to_writer_pretty, to_string_pretty};
use snowflake::SnowflakeIdBucket;
use std::collections::HashMap;
use std::fs::File;

#[derive(Debug, Default, Deserialize, Serialize)]
enum TaskState {
    Abandon,
    Done,
    #[default]
    Todo,
}

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    id: i64,
    content: String,
    state: TaskState,
    comments: Option<String>,
    create_time: String,
    update_time: String,
    deadline: Option<String>,
    child_tasks: Vec<Task>,
    parent_tasks: Vec<Task>,
}

impl Default for Task {
    fn default() -> Self {
        let mut id_generator_bucket = SnowflakeIdBucket::new(1, 1);
        let local_time = Local::now();
        Self {
            id: id_generator_bucket.get_id(),
            content: "".to_string(),
            state: TaskState::default(),
            comments: None,
            create_time: local_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            update_time: local_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            deadline: None,
            child_tasks: Vec::new(),
            parent_tasks: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TaskGroup {
    group_name: String,
    tasks: Vec<Task>,
}

impl TaskGroup {
    fn new(group_name: String) -> Self {
        Self {
            group_name,
            tasks: Vec::new(),
        }
    }
    // fn import(group_name: String) -> Self {
    //     Self {
    //         group_name: group_name,
    //         tasks: Vec::new()
    //     }
    // }
    // fn add(&mut self, content: String, comments: Option<String>, deadline: Option<String>) {
    fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }
    fn delete(&mut self, content: String) -> Result<(), &str> {
        for (index, task) in self.tasks.iter().enumerate() {
            if task.content == content {
                self.tasks.remove(index);
                return Ok(());
            }
        }
        Err("Task not found")
    }
    fn change_state(&mut self, content: String, state: TaskState) -> Result<(), &str> {
        for (index, task) in self.tasks.iter().enumerate() {
            if task.content == content {
                self.tasks[index].state = state;
                let local_time = Local::now();
                self.tasks[index].update_time = local_time.format("%Y-%m-%d %H:%M:%S").to_string();
                return Ok(());
            }
        }
        Err("Task not found")
    }
}

fn main() -> Result<(), ()> {
    let mut task_groups = HashMap::new();
    task_groups.insert(
        String::from("homeless"),
        TaskGroup::new("homeless".to_owned()),
    );
    if let Some(group) = task_groups.get_mut("homeless") {
        group.add(Task {
            content: "fasfdsasf".to_owned(),
            ..Default::default()
        })
    }
    if let Some(group) = task_groups.get_mut("homeless") {
        group.add(Task {
            content: "content".to_owned(),
            ..Default::default()
        })
    }
    println!("{:?}", task_groups.get("homeless"));
    if let Some(group) = task_groups.get_mut("homeless") {
        group
            .change_state("content".to_owned(), TaskState::Abandon)
            .unwrap();
    }
    println!("{:?}", task_groups.get("homeless"));
    if let Some(group) = task_groups.get_mut("homeless") {
        group.delete("content".to_owned()).unwrap();
    }
    println!("{:?}", task_groups.get("homeless"));
    match task_groups.get("homeless") {
        Some(group) => {
            let j = to_string_pretty(group).unwrap();
            let f = File::create("./tasks.json").unwrap();
            to_writer_pretty(f, &j).unwrap();
            println!("{}", j);
        }
        None => (),
    }

    Ok(())
}
