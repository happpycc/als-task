use crate::models::{App, InputMode, ScrollY, Task, Window};


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
}
