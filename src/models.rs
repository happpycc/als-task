use rusqlite::Connection;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Abandon,
    Done,
    Todo,
}

#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Insert(InsertPosistion),
}

#[derive(PartialEq)]
pub enum InsertPosistion {
    Previous,
    Next
}

#[derive(Debug, PartialEq)]
pub enum Window {
    Groups,
    Tasks,
}

#[derive(Debug)]
pub struct Task {
    pub depth: u8,
    pub content: String,
    pub task_state: State,
    pub group_state: State,
    pub create_time: String,
}

#[derive(Debug)]
pub struct TaskGroup {
    pub name: String,
    pub tasks: Vec<Task>,
    // Currently seleted task
    pub index: usize,
}

pub struct ScrollY {
    pub current: u16,
    pub max: i16,
}

pub struct App {
    pub task_groups: Vec<TaskGroup>,

    // Currently seleted group
    pub index: usize,

    // Horizontal movement
    pub scroll_task: ScrollY,
    pub scroll_group: ScrollY,

    // Database connection
    pub conn: Connection,

    // Input mode
    pub input_mode: InputMode,

    // Currently seleted window
    pub window: Window,
}
