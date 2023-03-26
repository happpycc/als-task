
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

    pub fn scroll_left(&mut self) {
        if self.scroll.current == 0 {
            if self.scroll.max > 0 {self.scroll.current = self.scroll.max as u16}
            else {self.scroll.current = 0}
        }
        else {self.scroll.current -= 1;}
    }

    pub fn scroll_right(&mut self) {
        if self.scroll.max > 0 {
            if self.scroll.current as i16 >= self.scroll.max {self.scroll.current = 0}
            else {self.scroll.current += 1}
        } else {self.scroll.current = 0}
    }
}
