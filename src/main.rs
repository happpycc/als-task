#[derive(Debug)]
enum taskState {
    abandon,
    done,
    none,
}

#[derive(Debug)]
struct Task {
    content: String,
    status: taskState,
    comments: Option<String>,
    create_time: String,
    update_time: String,
}

#[derive(Debug)]
struct TaskGroup {
    tasks: Vec<Task>,
}

impl TaskGroup {
    fn new() -> Self {
        Self {
            tasks: Vec::new()
        }
    }
    fn import() -> Self {
        Self {
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
    println!("Hello, Wolrd");
}
