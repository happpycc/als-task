use rusqlite::Connection;

use crate::{models::{TaskGroup, Task, InputMode, InsertPosistion}, database::tasks::{
    insert_task,
    delete_task,
    update_task
}};

impl TaskGroup {
    pub fn add_brother_next(&mut self, depth: u8) {
        if self.tasks.len() != 0 { self.index += 1 };
        self.tasks.insert(self.index, Task {
            depth,
            ..Default::default()
        });
    }

    pub fn add_brother_prev(&mut self) {
        self.tasks.insert(self.index, Task::new());
    }

    pub fn add_finished(
        &mut self,
        conn: &Connection,
        input_mode: &InputMode,
        content: &[String]) {
        let content = &content[0];

        // If content == "" return 
        if content == "" {
            return self.add_abandoned()
        }

        self.tasks[self.index].content = content.to_string();

        match input_mode {
            InputMode::Normal => {},
            InputMode::Insert(position) => {
                let task = &self.tasks[self.index];
                match position {
                    InsertPosistion::Current => {
                        update_task(
                            conn,
                            &self.name,
                            &task
                        ).unwrap();
                    },
                    _ => {
                        insert_task(
                            conn,
                            &self.name,
                            task,
                            &self.tasks,
                            self.index
                        ).unwrap()
                    }
                }
            }
        }
    }

    pub fn add_abandoned(&mut self) {
        
    }

    pub fn delete_current(&mut self, conn: &Connection) {
        if self.tasks.len() == 0 { return }

        delete_task(
            conn,
            self.tasks.len(),
            &self.name,
            self.index,
            self.tasks[self.index].create_time
        ).unwrap();
        
        self.tasks.remove(self.index);

        if self.index == self.tasks.len() && self.index != 0 {
            self.index -= 1
        }
    }
}
