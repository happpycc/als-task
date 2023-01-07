use rusqlite::Connection;

#[derive(Debug, Default, Clone, Copy)]
pub enum TaskState {
    Abandon,
    Done,
    #[default]
    Todo,
}

pub enum InputMode {
    Normal,
    Insert,
}

#[derive(Debug)]
pub struct Task {
    pub depth: u8,
    pub content: String,
    pub state: TaskState,
    pub create_time: Option<String>
}

pub struct TaskGroup {
    pub group_name: String,
    pub tasks: Vec<Task>,
}

pub struct SeletedIndex {
    pub task: usize,
    pub group: usize,
}

pub struct ScrollY {
    pub current: u16,
    pub max: i16,
}

pub struct App {
    pub task_groups: Vec<TaskGroup>,

    // Currently seleted task or group
    pub index: SeletedIndex,

    // Horizontal movement
    pub scroll_task: ScrollY,
    pub scroll_group: ScrollY,

    // Database connection
    pub conn: Connection,

    // Input mode
    pub input_mode: InputMode,
}
