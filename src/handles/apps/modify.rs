use crate::models::{App, InputMode};


impl App {
    // Add group next
    pub fn add_brother_next(&mut self) {
        self.input_mode = InputMode::Insert;
    }

    // Add group prev
    pub fn add_brother_prev(&mut self) {
        self.input_mode = InputMode::Insert;
    }
}
