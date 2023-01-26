
use crate::models::TaskGroup;

impl TaskGroup {
    pub fn index_next(&mut self) {
        if self.index == if self.tasks.len() != 0 {self.tasks.len() - 1} else {0} {
            self.index = 0;
            return;
        }
        self.index += 1;
    }

    pub fn index_prev(&mut self) {
        if self.index == 0 {
            self.index = if self.tasks.len() != 0 {
                self.tasks.len() - 1
            } else {
                0
            };
            return;
        }
        self.index -= 1;
    }
}
