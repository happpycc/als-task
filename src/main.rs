#[derive(Debug)]
enum TaskState{
    Abandon,
    Done,
    Todo,
}

#[derive(Debug)]
struct Task {
    content: String,
    status: TaskState,
    comments: Option<String>,
    create_time: String,
    update_time: String,
    deadline: String,
}

#[derive(Debug)]
struct TaskGroup {
    group_name: String,
    tasks: Vec<Task>,
}

impl TaskGroup {
    fn new(group_name: String) -> Self {
        Self {
            group_name: group_name,
            tasks: Vec::new()
        }
    }
    fn import(group_name: String) -> Self {
        Self {
            group_name: group_name,
            tasks: Vec::new()
        }
    }
    fn add(content: String, comments: Option<String>) -> bool {
        true
    }
    fn delete(id: i32) -> bool {
        true
    }
}

fn main() {
    let mut task_groups: Vec<TaskGroup> = vec![];
    let homeless_group = TaskGroup::new("homeless".to_owned());
    task_groups.push(homeless_group);
    println!("{:?}", task_groups);
}
