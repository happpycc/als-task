use std::collections::HashMap;
use chrono::prelude::{DateTime, Local};

#[derive(Debug)]
enum TaskState{
    Abandon,
    Done,
    Todo,
}

#[derive(Debug)]
struct Task {
    content: String,
    state: TaskState,
    comments: Option<String>,
    create_time: DateTime<Local>,
    update_time: DateTime<Local>,
    deadline: Option<String>,
}

#[derive(Debug)]
struct TaskGroup {
    group_name: String,
    tasks: Vec<Task>,
}

impl TaskGroup {
    fn new(group_name: String) -> Self {
        Self {
            group_name,
            tasks: Vec::new()
        }
    }
    // fn import(group_name: String) -> Self {
    //     Self {
    //         group_name: group_name,
    //         tasks: Vec::new()
    //     }
    // }
    fn add(mut self, content: String, comments: Option<String>, deadline: Option<String>) {
       self.tasks.push(Task {
            content,
            state: TaskState::Todo,
            comments,
            create_time: Local::now(),
            update_time: Local::now(),
            deadline,
       });
    }
    fn delete(id: i32) -> bool {
        true
    }
}

fn main() {
    let mut task_groups = HashMap::new();
    task_groups.insert(String::from("homeless"), TaskGroup::new("homeless".to_owned()));
    println!("{:?}", task_groups);
    let homeless_group = task_groups.get(&"homeless".to_owned());
    match homeless_group {
        Some(group) => group.add("aaa".to_owned(), Some("fasdfdsaf".to_owned()), None),
        None => (),
    }
    println!("{:?}", homeless_group)
}
