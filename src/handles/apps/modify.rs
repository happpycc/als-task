use rusqlite::params;

use crate::models::{App, InputMode, TaskGroup, InsertPosistion};
use crate::database::create_group;


impl App {
    // Add group next
    pub fn add_brother_next(&mut self) {
        self.input_mode = InputMode::Insert(InsertPosistion::Next);
        if self.task_groups.len() != 0 { self.index += 1 };
        self.task_groups.insert(self.index, TaskGroup::new("".to_string()));
    }

    // Add group prev
    pub fn add_brother_prev(&mut self) {
        self.input_mode = InputMode::Insert(InsertPosistion::Previous);
        self.task_groups.insert(self.index, TaskGroup::new("".to_string()));
    }

    pub fn add_finished(&mut self, name: &[String]) {
        let name = &name[0];
        if name == "" { return self.add_abandoned() }
        self.input_mode = InputMode::Normal;
        self.task_groups[self.index].name = name.to_string();

        // Change after current task_group's task_groups' id
        for index in self.index + 1..self.task_groups.len() {
            self.conn.execute(
                "UPDATE groups SET group_id = ?1 WHERE create_time = ?2",
                params![index, self.task_groups[index].create_time])
            .unwrap();
        }

        // Add task_groups into groups table
        let task_group = &self.task_groups[self.index];
        self.conn.execute("
            INSERT INTO groups (
                group_id,
                name,
                create_time
            ) VALUES (?1, ?2, ?3);",
        params![
            self.index,
            task_group.name,
            task_group.create_time,
        ]).unwrap();

        // Create table => task_groups
        create_group(&self.conn, &task_group.name).unwrap();
    }

    // When insert want to abandoned
    pub fn add_abandoned(&mut self) {
        self.task_groups.remove(self.index);
        match &self.input_mode {
            InputMode::Insert(position) => match &position {
                InsertPosistion::Next => {
                    self.index -= if self.task_groups.len() == 0 {0} else {1}
                }
                InsertPosistion::Previous => {}
            }
            InputMode::Normal => {}
        }
        self.input_mode = InputMode::Normal;
    }
}
