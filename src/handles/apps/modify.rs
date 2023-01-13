use crate::models::{App, InputMode, TaskGroup, InsertPosistion};


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

    pub fn add_finished(&mut self) {
        self.input_mode = InputMode::Normal;
    }

    // When insert want to abandoned
    pub fn add_abandoned(&mut self) {
        self.task_groups.remove(self.index);
        match &self.input_mode {
            InputMode::Insert(position) => match &position {
                InsertPosistion::Next => { self.index -= 1 }
                InsertPosistion::Previous => {}
            }
            InputMode::Normal => {}
        }
        self.input_mode = InputMode::Normal;
    }
}
