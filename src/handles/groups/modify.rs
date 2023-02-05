use rusqlite::Connection;

use crate::{models::{TaskGroup, Task, InputMode, InsertPosistion}, database::tasks::insert_task};

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

        let old_content = self.tasks[self.index].content.clone();
        self.tasks[self.index].content = content.to_string();


        match input_mode {
            InputMode::Normal => {},
            InputMode::Insert(position) => {
                let task = &self.tasks[self.index];
                match position {
                    InsertPosistion::Current => {},
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

    pub fn delete_current(&mut self) {
        
    }
}
