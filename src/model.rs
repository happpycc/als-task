use tui::layout::Rect;
use rusqlite::Connection;


#[derive(Debug, Default)]
pub enum TaskState {
    Abandon,
    Done,
    #[default]
    Todo,
}

pub enum InputMode {
    Normal,
    Editing,
}

#[derive(Debug)]
pub struct Task {
    pub task_id: i64,
    pub depth: u8,
    pub content: String,
    pub state: TaskState,
    pub comments: Option<String>,
    pub create_time: String,
    pub update_time: String,
    pub dead_time: Option<String>,
    pub next_task: Option<i64>
}

pub struct App {
    pub tasks: Vec<Task>,
    // Current selected value of the tasks
    pub index: usize,
    // Current input mode
    pub input_mode: InputMode,
    pub window_rect: Rect,
    pub scroll: u16,
    pub conn: Connection,
}