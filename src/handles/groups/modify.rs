use rusqlite::Connection;

use crate::{
    database::tasks::{change_task_state, delete_task, insert_task, update_task},
    models::{InputMode, InsertPosistion, Task, TaskGroup, State},
};

impl TaskGroup {
    pub fn add_brother_next(&mut self, depth: u8) {
        if self.tasks.len() != 0 {
            self.index += 1
        };
        self.tasks.insert(
            self.index,
            Task {
                depth,
                ..Default::default()
            },
        );
    }

    pub fn add_brother_prev(&mut self) {
        self.tasks.insert(self.index, Task::new());
    }

    pub fn add_finished(&mut self, conn: &Connection, input_mode: &InputMode, content: &[String]) {
        let content = &content[0];

        // If content == "" return
        if content.trim() == "" {
            return self.add_abandoned(input_mode);
        }

        self.tasks[self.index].content = content.to_string();

        match input_mode {
            InputMode::Normal => {}
            InputMode::Insert(position) => {
                let task = &self.tasks[self.index];
                match position {
                    InsertPosistion::Current => {
                        update_task(conn, &self.name, &task).unwrap();
                    }
                    _ => insert_task(conn, &self.name, task, &self.tasks, self.index).unwrap(),
                }
            }
        }
    }

    pub fn add_abandoned(&mut self, input_mode: &InputMode) {
        match input_mode {
            InputMode::Normal => {}
            InputMode::Insert(position) => match &position {
                InsertPosistion::Next => {
                    self.tasks.remove(self.index);
                    self.index -= if self.tasks.len() == 0 { 0 } else { 1 }
                }
                InsertPosistion::Previous => {
                    self.tasks.remove(self.index);
                }
                InsertPosistion::Current => {}
            },
        }
    }

    pub fn delete_current(&mut self, conn: &Connection) {
        if self.tasks.len() == 0 {
            return;
        }

        delete_task(
            conn,
            self.tasks.len(),
            &self.name,
            self.index,
            self.tasks[self.index].create_time,
        )
        .unwrap();

        self.tasks.remove(self.index);

        if self.index == self.tasks.len() && self.index != 0 {
            self.index -= 1
        }
    }

    pub fn change_task_state(&mut self, conn: &Connection) {
        if self.tasks.len() == 0 {
            return;
        }

        self.tasks[self.index].task_state = match &self.tasks[self.index].task_state {
            State::Todo => State::Done,
            State::Done => State::Todo,
            State::Abandon => State::Todo
        };

        change_task_state(
            conn, 
            &self.name,
            self.tasks[self.index].task_state,
            self.tasks[self.index].create_time
        ).unwrap();
    }

    pub fn task_state_abandon(&mut self, conn: &Connection) {
        if self.tasks.len() == 0 {
            return;
        }

        self.tasks[self.index].task_state = match &self.tasks[self.index].task_state {
            State::Abandon => State::Todo,
            State::Done => State::Abandon,
            State::Todo => State::Abandon
        };

        change_task_state(
            conn, 
            &self.name,
            self.tasks[self.index].task_state,
            self.tasks[self.index].create_time
        ).unwrap();
    }
}
