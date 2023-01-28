use rusqlite::params;

use crate::models::{App, InputMode, TaskGroup, InsertPosistion};
use crate::database::groups::{insert_group, update_group, delete_group};


impl App {
    // If there is repeated
    fn repeat_or_not(&mut self, name: &String) -> bool {
        for group in &self.task_groups {
            if &group.name == name { return false }
        }
        true
    }
    
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

        // If name == "" return 
        // If name == "groups" return 
        // If has name return
        if name == ""
            || name == "groups"
            || name == "homeless"
            || !self.repeat_or_not(name)
        {
            return self.add_abandoned()
        }

        let old_name = self.task_groups[self.index].name.clone();
        self.task_groups[self.index].name = name.to_string();


        match &self.input_mode {
            InputMode::Normal => {},
            InputMode::Insert(position) => {
                let task_group = &self.task_groups[self.index];
                match position {
                    InsertPosistion::Current => {
                        update_group(
                            &self.conn,
                            task_group,
                            &old_name
                        ).unwrap();
                    },
                    _ => {
                        insert_group(
                            &self.conn,
                            task_group,
                            &self.task_groups,
                            self.index,
                            self.task_groups.len()
                        ).unwrap()
                    }
                }
            }
        }

        self.input_mode = InputMode::Normal;
    }

    // When insert want to abandoned
    pub fn add_abandoned(&mut self) {
        match &self.input_mode {
            InputMode::Normal => {}
            InputMode::Insert(position) => match &position {
                InsertPosistion::Next => {
                    self.task_groups.remove(self.index);
                    self.index -= if self.task_groups.len() == 0 {0} else {1}
                }
                InsertPosistion::Previous => {
                    self.task_groups.remove(self.index);
                },
                InsertPosistion::Current => {}
            }
        }
        self.input_mode = InputMode::Normal;
    }

    // Delete task_group 
    pub fn delete_current(&mut self) {
        // If not group in groups
        if self.task_groups[self.index].name == "homeless" {
            return; 
        }

        // Delete current group in database 
        delete_group(
            &self.conn,
            &self.task_groups[self.index],
            &self.task_groups,
            self.index,
            self.task_groups.len()
        ).unwrap();

        // Delete current group in program 
        self.task_groups.remove(self.index);
        self.index -= if self.index == 0 {0} else {1}
    }
}
