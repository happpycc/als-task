use crate::models::TaskGroup;


impl TaskGroup {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tasks: vec![],
            index: 0,
        }
    }
}
