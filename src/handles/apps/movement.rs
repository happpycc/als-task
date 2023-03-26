use crate::models::{App, Window};


impl App {
    pub fn index_next(&mut self) {
        match self.window {
            Window::Groups => {
                if self.index == if self.task_groups.len() != 0 {self.task_groups.len() - 1} else {0} {
                    self.index = 0;
                    return;
                }
                self.index += 1;
            },
            Window::Tasks => {
                let task_group = &self.task_groups[self.index];
                if task_group.index == if task_group.tasks.len() != 0 {task_group.tasks.len() - 1} else {0} {
                    self.task_groups[self.index].index = 0;
                    return;
                }
                self.task_groups[self.index].index += 1;
            },
        }
    }

    pub fn index_prev(&mut self) {
        match self.window {
            Window::Groups => {
                if self.index == 0 {
                    self.index = if self.task_groups.len() != 0 {self.task_groups.len() - 1} else {0};
                    return;
                }
                self.index -= 1;
            },
            Window::Tasks => {
                let task_group = &self.task_groups[self.index];
                if task_group.index == 0 {
                    self.task_groups[self.index].index =
                        if task_group.tasks.len() != 0 {task_group.tasks.len() - 1} else {0};
                    return;
                }
                self.task_groups[self.index].index -= 1;
            },
        }
    }

    pub fn window_change(&mut self) {
        match self.window {
            Window::Groups => { self.window = Window::Tasks },
            Window::Tasks => { self.window = Window::Groups }
        }
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
