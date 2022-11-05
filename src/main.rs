use std::collections::HashMap;
use chrono::prelude::{DateTime, Local};
use std::io;

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
    fn add(&mut self, content: String, comments: Option<String>, deadline: Option<String>) {
       self.tasks.push(Task {
            content,
            state: TaskState::Todo,
            comments,
            create_time: Local::now(),
            update_time: Local::now(),
            deadline,
       });
    }
    fn delete(&mut self, content: String) {
        for (index, task) in self.tasks.iter().enumerate() {
            if task.content == content {
                self.tasks.remove(index);
                return
            }
        }
    }
    fn changeState(&mut self, content: String, state: TaskState) {
        for (index, task) in self.tasks.iter().enumerate() {
            if task.content == content {
                self.tasks[index].state = state;
                return
            }
        }
    }
}

fn main() {
    let mut task_groups = HashMap::new();
    task_groups.insert(String::from("homeless"), TaskGroup::new("homeless".to_owned()));
    match task_groups.get_mut("homeless") {
        Some(group) => group.add("content".to_owned(), None, None),
        None => ()
    }
    println!("{:?}", task_groups.get("homeless"));
    match task_groups.get_mut("homeless") {
        Some(group) => group.changeState("content".to_owned(), TaskState::Abandon),
        None => ()
    }
    println!("{:?}", task_groups.get("homeless"));
    match task_groups.get_mut("homeless") {
        Some(group) => group.delete("content".to_owned()),
        None => ()
    }
    println!("{:?}", task_groups.get("homeless"));
}
