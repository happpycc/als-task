use rusqlite::params;
use tui::layout::Rect;
use unicode_width::UnicodeWidthStr;

use crate::models::{App, InputMode, Task, TaskState};

use crate::utils::others::{load_database, load_tasks};

impl App {
    pub fn new() -> App {
        let conn = load_database().unwrap();
        let tasks = load_tasks(&conn).unwrap();
        App {
            tasks,
            index: 0,
            input_mode: InputMode::Normal,
            window_rect: Rect::default(),
            scroll: 0,
            conn,
            scroll_right_max: 0,
        }
    }

    pub fn edit_finished(&mut self, content: &[String]) {
        let content = &content[0];
        self.input_mode = InputMode::Normal;
        if content == "" || format!("{:?} ", self.tasks[self.index].state).len() as u16 + content.width() as u16 + (self.tasks[self.index].depth * 4) as u16 > 80 {
            self.edit_abandon();
        } else {
            self.tasks[self.index].content = content.to_string();
            for index in (self.index + 1)..self.tasks.len() {
                self.tasks[index].task_id = index;
                self.conn.execute("UPDATE tasks SET task_id = ?1 WHERE create_time = ?2", params![index, self.tasks[index].create_time]).unwrap();
            }
            self.tasks[self.index].task_id = self.index;
            let task = &self.tasks[self.index];
            self.conn.execute("
                INSERT INTO tasks (
                    task_id,
                    depth,
                    content,
                    state,
                    comments,
                    create_time,
                    update_time,
                    dead_time
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8); 
            ", params![task.task_id, task.depth, task.content, format!("{:?}", task.state), task.comments,
            task.create_time, task.update_time, task.dead_time]).unwrap();
        }
    }

    pub fn edit_abandon(&mut self) {
        self.tasks.remove(self.index);
        self.index -= if self.index == 0 {0} else {1};
        self.input_mode = InputMode::Normal;
    }

    fn get_next_brother_task(&self, current_depth: u8) -> usize {
        if self.tasks.len() == 0 {return 0;}
        if self.index == self.tasks.len() - 1 {return self.tasks.len();}
        // else if self.tasks[self.index + 1].depth == self.tasks[self.index].depth {return self.index + 1;}
        for index in self.index + 1..self.tasks.len() {
            if current_depth >= self.tasks[index].depth {return index;}
            if index == self.tasks.len() - 1 {return index + 1;}
        }
        0
    }

    pub fn add_brother_task(&mut self) {
        self.input_mode = InputMode::Editing;
        let current_depth = if self.tasks.len() == 0 {0} else {self.tasks[self.index].depth};
        self.index = self.get_next_brother_task(current_depth);
        self.tasks.insert(self.index, Task { depth: current_depth, ..Default::default()});
    }

    pub fn add_child_task(&mut self) {
        self.input_mode = InputMode::Editing;
        self.tasks.insert(if self.tasks.len() == 0 {0} else {self.index + 1}, Task { depth: if self.tasks.len() == 0 {0} else {self.tasks[self.index].depth + 1}, ..Default::default() });
        self.index += if self.tasks.len() - 1 == 0 {0} else {1};
    }

    pub fn delete_task(&mut self) {
        if self.tasks.len() == 0 {return;}
        else if self.index == self.tasks.len() -1 {
            self.conn.execute("DELETE FROM tasks WHERE task_id=?1", params![self.tasks[self.index].task_id]).unwrap();
            self.tasks.remove(self.index);
            self.index -= if self.index == 0 {0} else {1};
            return;
        }
        let mut delete_num = 0;
        let first_index = self.tasks[self.index].task_id - if self.index == 0 {0} else {1};
        let first_depth = self.tasks[self.index].depth;
        let mut is_delete = true;
        for index in self.index..self.tasks.len() {
            if index != self.index && self.tasks[self.index].depth <= first_depth{is_delete = false;}
            if is_delete {
                delete_num += 1;
                self.conn.execute("DELETE FROM tasks WHERE create_time = ?1", params![self.tasks[self.index].create_time]).unwrap();
                self.tasks.remove(self.index);
            } else {
                self.tasks[self.index].task_id -= delete_num;
                self.conn.execute("UPDATE tasks SET task_id = ?1 WHERE create_time = ?2", params![self.tasks[self.index].task_id, self.tasks[self.index].create_time]).unwrap();
                self.index += 1;
            }
            self.index = first_index;
            // for index in self.index + 1..self.tasks.len() {
            // }
        }
    }

    pub fn change_state(&mut self, state: TaskState) {
        if self.tasks.len() == 0 {return}
        if self.index == self.tasks.len() -1 {
            self.tasks[self.index].state = state;
            self.conn.execute("UPDATE tasks SET state = ?1 WHERE create_time = ?2", params![format!("{:?}", self.tasks[self.index].state), self.tasks[self.index].create_time]).unwrap();
            return;
        }
        let current_depth = self.tasks[self.index].depth;
        for index in self.index..self.tasks.len() {
            if index != self.index && self.tasks[index].depth <= current_depth {return}
            self.tasks[index].state = state;
            self.conn.execute("UPDATE tasks SET state = ?1 WHERE create_time = ?2", params![format!("{:?}", self.tasks[index].state), self.tasks[index].create_time]).unwrap();
        }
    }

    pub fn next(&mut self) {
        if self.index == if self.tasks.len() != 0 {self.tasks.len() - 1} else {0} {
            self.index = 0;
            return;
        }
        self.index += 1;
    }

    pub fn previous(&mut self) {
        if self.index == 0 {
            self.index = if self.tasks.len() != 0 {self.tasks.len() - 1} else {0};
            return;
        }
        self.index -= 1;
    }

    pub fn scroll_left(&mut self) {
        if self.scroll == 0 {
            if self.scroll_right_max > 0 {self.scroll = self.scroll_right_max as u16}
            else {self.scroll = 0}
        }
        else {self.scroll -= 1;}
    }

    pub fn scroll_right(&mut self) {
        if self.scroll_right_max > 0 {
            if self.scroll as i16 == self.scroll_right_max {self.scroll = 0}
            else {self.scroll += 1}
        } else {self.scroll = 0}
        // if self.scroll_right_max <= 0 {self.scroll = 0}
        // else if self.scroll as i16 == self.scroll_right_max {self.scroll = 0}
        // else {self.scroll += 1}
    }
}